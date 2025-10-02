use std::collections::HashMap;
use std::time::SystemTime;

use der::{Decode, Encode, Reader};
use x509_cert::Certificate as X509CertCert;
// Using available const_oid constants based on actual const_oid 0.9 API
use const_oid::db::rfc5912::{SECP_224_R_1, SECP_256_R_1, SECP_384_R_1, SECP_521_R_1, ID_EC_PUBLIC_KEY};
use const_oid::db::rfc8410::{ID_X_25519, ID_X_448, ID_ED_25519, ID_ED_448};
use der::{AnyRef, Length, SliceReader, Tag};

use spki::AlgorithmIdentifier;

use super::super::errors::TlsError;
use super::super::types::ParsedCertificate;

/// Extract name attributes from x509-cert Name structure
pub fn extract_name_attributes(name: &x509_cert::name::Name, attrs: &mut HashMap<String, String>) {
    use der::asn1::{Ia5StringRef, PrintableStringRef, Utf8StringRef};

    // Common OIDs for DN components
    const OID_CN: &str = "2.5.4.3"; // commonName
    const OID_O: &str = "2.5.4.10"; // organizationName
    const OID_OU: &str = "2.5.4.11"; // organizationalUnitName
    const OID_C: &str = "2.5.4.6"; // countryName
    const OID_ST: &str = "2.5.4.8"; // stateOrProvinceName
    const OID_L: &str = "2.5.4.7"; // localityName

    // Iterate through RDNs (Relative Distinguished Names)
    for rdn in &name.0 {
        // Each RDN contains one or more AttributeTypeAndValue
        for atv in rdn.0.iter() {
            let oid_string = atv.oid.to_string();

            // Extract the value as string using proper ASN.1 type handling
            let string_value = if let Ok(ps) = PrintableStringRef::try_from(&atv.value) {
                Some(ps.to_string())
            } else if let Ok(utf8s) = Utf8StringRef::try_from(&atv.value) {
                Some(utf8s.to_string())
            } else if let Ok(ia5s) = Ia5StringRef::try_from(&atv.value) {
                Some(ia5s.to_string())
            } else {
                None
            };

            if let Some(value_str) = string_value {
                match oid_string.as_str() {
                    OID_CN => {
                        attrs.insert("CN".to_string(), value_str);
                    }
                    OID_O => {
                        attrs.insert("O".to_string(), value_str);
                    }
                    OID_OU => {
                        attrs.insert("OU".to_string(), value_str);
                    }
                    OID_C => {
                        attrs.insert("C".to_string(), value_str);
                    }
                    OID_ST => {
                        attrs.insert("ST".to_string(), value_str);
                    }
                    OID_L => {
                        attrs.insert("L".to_string(), value_str);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Extract Subject Alternative Names from certificate extension
fn extract_subject_alt_names(ext: &x509_cert::ext::Extension) -> (Vec<String>, Vec<std::net::IpAddr>) {
    let mut san_dns_names = Vec::new();
    let mut san_ip_addresses = Vec::new();

    // Parse SubjectAltName extension properly using ASN.1
    use der::{Decode, Reader, SliceReader, Tag, TagNumber};

    let ext_data = ext.extn_value.as_bytes();

    // Parse the OCTET STRING wrapper first
    match der::asn1::OctetString::from_der(ext_data) {
        Ok(octet_string) => {
            // Now parse the actual SubjectAltName SEQUENCE
            let san_bytes = octet_string.as_bytes();
            let mut reader = SliceReader::new(san_bytes).ok();

            if let Some(mut reader) = reader {
                // Expect SEQUENCE tag (0x30)
                if let Ok(sequence_header) = reader.peek_header() {
                    if sequence_header.tag == Tag::Sequence {
                        let _ = reader.read_header(); // Consume the SEQUENCE header

                        // Parse each GeneralName in the sequence
                        while !reader.is_finished() {
                            if let Ok(name_header) = reader.peek_header() {
                                let tag_number = name_header.tag.number();

                                match tag_number {
                                    TagNumber::N2 => {
                                        // dNSName [2] IA5String
                                        let _ = reader.read_header();
                                        if let Ok(dns_bytes) = reader.read_slice(name_header.length) {
                                            if let Ok(dns_name) = String::from_utf8(dns_bytes.to_vec()) {
                                                san_dns_names.push(dns_name);
                                            }
                                        }
                                    }
                                    TagNumber::N7 => {
                                        // iPAddress [7] OCTET STRING
                                        let _ = reader.read_header();
                                        if let Ok(ip_bytes) = reader.read_slice(name_header.length) {
                                            match ip_bytes.len() {
                                                4 => {
                                                    // IPv4 address
                                                    let mut octets = [0u8; 4];
                                                    octets.copy_from_slice(ip_bytes);
                                                    san_ip_addresses.push(std::net::IpAddr::V4(
                                                        std::net::Ipv4Addr::from(octets),
                                                    ));
                                                }
                                                16 => {
                                                    // IPv6 address
                                                    let mut octets = [0u8; 16];
                                                    octets.copy_from_slice(ip_bytes);
                                                    san_ip_addresses.push(std::net::IpAddr::V6(
                                                        std::net::Ipv6Addr::from(octets),
                                                    ));
                                                }
                                                _ => {
                                                    // Invalid IP address length
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        // Skip other GeneralName types
                                        let _ = reader.peek_header();
                                        let _ = reader.read_slice(name_header.length);
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            // Fall back to simple string search if ASN.1 parsing fails
            let ext_string = String::from_utf8_lossy(ext_data);
            if ext_string.contains("DNS:") {
                for part in ext_string.split(',') {
                    let part = part.trim();
                    if let Some(dns_name) = part.strip_prefix("DNS:") {
                        san_dns_names.push(dns_name.to_string());
                    }
                }
            }
        }
    }

    (san_dns_names, san_ip_addresses)
}

/// Extract Basic Constraints from certificate extension
fn extract_basic_constraints(ext: &x509_cert::ext::Extension) -> bool {
    // Parse BasicConstraints extension
    use der::{Decode, Reader, SliceReader, Tag};

    let ext_data = ext.extn_value.as_bytes();

    // Parse the OCTET STRING wrapper first
    if let Ok(octet_string) = der::asn1::OctetString::from_der(ext_data) {
        let bc_bytes = octet_string.as_bytes();
        if let Ok(mut reader) = SliceReader::new(bc_bytes) {
            // Expect SEQUENCE tag (0x30)
            if let Ok(sequence_header) = reader.peek_header() {
                if sequence_header.tag == Tag::Sequence {
                    let _ = reader.read_header(); // Consume the SEQUENCE header

                    // Look for BOOLEAN tag (0x01)
                    if !reader.is_finished() {
                        if let Ok(bool_header) = reader.peek_header() {
                            if bool_header.tag == Tag::Boolean {
                                let _ = reader.read_header();
                                if let Ok(bool_bytes) = reader.read_slice(bool_header.length) {
                                    return !bool_bytes.is_empty() && bool_bytes[0] != 0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

/// Extract Key Usage from certificate extension
fn extract_key_usage(ext: &x509_cert::ext::Extension) -> Vec<String> {
    let mut key_usage = Vec::new();

    // Parse KeyUsage extension
    use der::{Decode, Reader, SliceReader, Tag};

    let ext_data = ext.extn_value.as_bytes();

    // Parse the OCTET STRING wrapper first
    if let Ok(octet_string) = der::asn1::OctetString::from_der(ext_data) {
        let ku_bytes = octet_string.as_bytes();
        if let Ok(mut reader) = SliceReader::new(ku_bytes) {
            // Expect BIT STRING tag (0x03)
            if let Ok(bitstring_header) = reader.peek_header() {
                if bitstring_header.tag == Tag::BitString {
                    let _ = reader.read_header(); // Consume the BIT STRING header

                    if let Ok(bitstring_bytes) = reader.read_slice(bitstring_header.length) {
                        if !bitstring_bytes.is_empty() {
                            // First byte indicates unused bits
                            let _unused_bits = bitstring_bytes[0];
                            let bits = &bitstring_bytes[1..];

                            if !bits.is_empty() {
                                let byte0 = bits[0];

                                // KeyUsage bits (from RFC 5280):
                                if (byte0 & 0x80) != 0 { key_usage.push("digitalSignature".to_string()); }
                                if (byte0 & 0x40) != 0 { key_usage.push("nonRepudiation".to_string()); }
                                if (byte0 & 0x20) != 0 { key_usage.push("keyEncipherment".to_string()); }
                                if (byte0 & 0x10) != 0 { key_usage.push("dataEncipherment".to_string()); }
                                if (byte0 & 0x08) != 0 { key_usage.push("keyAgreement".to_string()); }
                                if (byte0 & 0x04) != 0 { key_usage.push("keyCertSign".to_string()); }
                                if (byte0 & 0x02) != 0 { key_usage.push("cRLSign".to_string()); }
                                if (byte0 & 0x01) != 0 { key_usage.push("encipherOnly".to_string()); }

                                // Check for second byte if present
                                if bits.len() > 1 {
                                    let byte1 = bits[1];
                                    if (byte1 & 0x80) != 0 { key_usage.push("decipherOnly".to_string()); }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    key_usage
}

/// Extract validity dates from certificate
fn extract_validity_dates(cert: &X509CertCert) -> (SystemTime, SystemTime) {
    let validity = &cert.tbs_certificate.validity;
    let not_before = validity.not_before.to_system_time();
    let not_after = validity.not_after.to_system_time();
    (not_before, not_after)
}