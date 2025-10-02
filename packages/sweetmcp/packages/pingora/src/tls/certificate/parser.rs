//! Internal certificate parsing implementation details

use std::collections::HashMap;
use std::convert::TryInto;
use std::net::IpAddr;

use der::{Decode, Encode, asn1::BitStringRef};
use x509_cert::Certificate as X509CertCert;
use x509_cert::ext::Extension;
use x509_cert::ext::pkix::{
    AuthorityInfoAccessSyntax, BasicConstraints, KeyUsage, SubjectAltName, name::GeneralName,
};

// Using available const_oid constants
use const_oid::db::rfc5912::ID_EC_PUBLIC_KEY;
use const_oid::db::rfc8410::{ID_ED_448, ID_ED_25519, ID_X_448, ID_X_25519};

use super::super::errors::TlsError;
use super::super::types::ParsedCertificate;

// Standard X.509 extension OID strings
const OID_SUBJECT_ALT_NAME: &str = "2.5.29.17";
const OID_BASIC_CONSTRAINTS: &str = "2.5.29.19";
const OID_KEY_USAGE: &str = "2.5.29.15";
const OID_AUTHORITY_INFO_ACCESS: &str = "1.3.6.1.5.5.7.1.1";

// Authority Information Access method OID strings
const OID_OCSP: &str = "1.3.6.1.5.5.7.48.1";
const OID_CA_ISSUERS: &str = "1.3.6.1.5.5.7.48.2";

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

/// Parse Subject Alternative Name extension
fn parse_subject_alt_name(extension: &Extension) -> Result<(Vec<String>, Vec<IpAddr>), TlsError> {
    let san = SubjectAltName::from_der(extension.extn_value.as_bytes())
        .map_err(|e| TlsError::CertificateParsing(format!("Failed to parse SAN extension: {e}")))?;

    let mut dns_names = Vec::new();
    let mut ip_addresses = Vec::new();

    for name in &san.0 {
        match name {
            GeneralName::DnsName(dns_name) => {
                dns_names.push(dns_name.to_string());
            }
            GeneralName::IpAddress(ip_octets) => {
                // Parse IP address from octets
                let octets = ip_octets.as_bytes();
                match octets.len() {
                    4 => {
                        // IPv4
                        if let Ok(ipv4) = TryInto::<[u8; 4]>::try_into(octets) {
                            ip_addresses.push(IpAddr::from(ipv4));
                        }
                    }
                    16 => {
                        // IPv6
                        if let Ok(ipv6) = TryInto::<[u8; 16]>::try_into(octets) {
                            ip_addresses.push(IpAddr::from(ipv6));
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok((dns_names, ip_addresses))
}

/// Parse Basic Constraints extension for CA flag
fn parse_basic_constraints(extension: &Extension) -> Result<bool, TlsError> {
    let basic_constraints =
        BasicConstraints::from_der(extension.extn_value.as_bytes()).map_err(|e| {
            TlsError::CertificateParsing(format!("Failed to parse BasicConstraints: {e}"))
        })?;

    Ok(basic_constraints.ca)
}

/// Parse Key Usage extension
fn parse_key_usage(extension: &Extension) -> Result<Vec<String>, TlsError> {
    let key_usage = KeyUsage::from_der(extension.extn_value.as_bytes())
        .map_err(|e| TlsError::CertificateParsing(format!("Failed to parse KeyUsage: {e}")))?;

    let mut usages = Vec::new();

    if key_usage.digital_signature() {
        usages.push("digital_signature".to_string());
    }
    if key_usage.non_repudiation() {
        usages.push("non_repudiation".to_string());
    }
    if key_usage.key_encipherment() {
        usages.push("key_encipherment".to_string());
    }
    if key_usage.data_encipherment() {
        usages.push("data_encipherment".to_string());
    }
    if key_usage.key_agreement() {
        usages.push("key_agreement".to_string());
    }
    if key_usage.key_cert_sign() {
        usages.push("key_cert_sign".to_string());
    }
    if key_usage.crl_sign() {
        usages.push("crl_sign".to_string());
    }
    if key_usage.encipher_only() {
        usages.push("encipher_only".to_string());
    }
    if key_usage.decipher_only() {
        usages.push("decipher_only".to_string());
    }

    Ok(usages)
}

/// Parse Authority Information Access extension for OCSP and CRL URLs
fn parse_authority_info_access(
    extension: &Extension,
) -> Result<(Vec<String>, Vec<String>), TlsError> {
    let aia = AuthorityInfoAccessSyntax::from_der(extension.extn_value.as_bytes())
        .map_err(|e| TlsError::CertificateParsing(format!("Failed to parse AIA extension: {e}")))?;

    let mut ocsp_urls = Vec::new();
    let mut crl_urls = Vec::new();

    for access_desc in &aia.0 {
        if let GeneralName::UniformResourceIdentifier(uri) = &access_desc.access_location {
            let url = uri.to_string();
            let method_str = access_desc.access_method.to_string();

            if method_str == OID_OCSP {
                ocsp_urls.push(url);
            } else if method_str == OID_CA_ISSUERS {
                crl_urls.push(url);
            }
        }
    }

    Ok((ocsp_urls, crl_urls))
}

/// Extract key size from public key information
fn extract_key_size(cert: &X509CertCert) -> Option<u32> {
    let public_key_info = &cert.tbs_certificate.subject_public_key_info;
    let algorithm_oid = public_key_info.algorithm.oid.to_string();

    match algorithm_oid.as_str() {
        "1.2.840.113549.1.1.1" => {
            // RSA - estimate key size from public key length
            let key_bits = BitStringRef::from(&public_key_info.subject_public_key);
            let bit_len = key_bits.raw_bytes().len() * 8;
            // Common RSA key sizes
            if bit_len >= 4000 {
                Some(4096)
            } else if bit_len >= 3000 {
                Some(3072)
            } else if bit_len >= 2000 {
                Some(2048)
            } else if bit_len >= 1000 {
                Some(1024)
            } else {
                None
            }
        }
        _ if algorithm_oid == ID_EC_PUBLIC_KEY.to_string() => {
            // ECDSA - estimate based on key length
            let key_bits = BitStringRef::from(&public_key_info.subject_public_key);
            let key_bytes = key_bits.raw_bytes().len();
            match key_bytes {
                97 | 49 => Some(384),  // P-384 (uncompressed | compressed)
                133 | 67 => Some(521), // P-521 (uncompressed | compressed)
                // P-256 (uncompressed | compressed) or any unknown key size defaults to P-256
                _ => Some(256),
            }
        }
        _ if algorithm_oid == ID_ED_25519.to_string()
            || algorithm_oid == ID_X_25519.to_string() =>
        {
            Some(255)
        } // Ed25519 or X25519
        _ if algorithm_oid == ID_ED_448.to_string() || algorithm_oid == ID_X_448.to_string() => {
            Some(448)
        } // Ed448 or X448
        _ => None,
    }
}

/// Parse certificate from `X509Certificate` struct to extract actual certificate information
pub fn parse_x509_certificate_from_der_internal(
    cert: &X509CertCert,
) -> Result<ParsedCertificate, TlsError> {
    // Extract subject DN using x509-cert API
    let mut subject = HashMap::new();
    extract_name_attributes(&cert.tbs_certificate.subject, &mut subject);

    // Extract issuer DN using x509-cert API
    let mut issuer = HashMap::new();
    extract_name_attributes(&cert.tbs_certificate.issuer, &mut issuer);

    // Extract validity dates
    let validity = &cert.tbs_certificate.validity;
    let not_before = validity.not_before.to_system_time();
    let not_after = validity.not_after.to_system_time();

    // Get raw DER bytes for OCSP validation
    let subject_der = cert
        .tbs_certificate
        .subject
        .to_der()
        .map_err(|e| TlsError::CertificateParsing(format!("Failed to encode subject: {e}")))?;

    let public_key_der = cert
        .tbs_certificate
        .subject_public_key_info
        .to_der()
        .map_err(|e| TlsError::CertificateParsing(format!("Failed to encode public key: {e}")))?;

    // Extract serial number
    let serial_number = cert.tbs_certificate.serial_number.as_bytes().to_vec();

    // Extract key algorithm information
    let algorithm = &cert.tbs_certificate.subject_public_key_info.algorithm;
    let algorithm_oid_str = algorithm.oid.to_string();
    let key_algorithm = match algorithm_oid_str.as_str() {
        "1.2.840.113549.1.1.1" => "RSA".to_string(),
        "1.2.840.10040.4.1" => "DSA".to_string(),
        "1.2.840.10046.2.1" => "DH".to_string(),
        _ if algorithm_oid_str == ID_EC_PUBLIC_KEY.to_string() => "ECDSA".to_string(),
        _ if algorithm_oid_str == ID_X_25519.to_string() => "X25519".to_string(),
        _ if algorithm_oid_str == ID_X_448.to_string() => "X448".to_string(),
        _ if algorithm_oid_str == ID_ED_25519.to_string() => "Ed25519".to_string(),
        _ if algorithm_oid_str == ID_ED_448.to_string() => "Ed448".to_string(),
        _ => "Unknown".to_string(),
    };

    // Extract key size
    let key_size = extract_key_size(cert);

    // Initialize extension-derived fields with defaults
    let mut san_dns_names = Vec::new();
    let mut san_ip_addresses = Vec::new();
    let mut is_ca = false;
    let mut key_usage = Vec::new();
    let mut ocsp_urls = Vec::new();
    let mut crl_urls = Vec::new();

    // Parse extensions if present
    if let Some(extensions) = &cert.tbs_certificate.extensions {
        for extension in extensions {
            let oid_str = extension.extn_id.to_string();

            match oid_str.as_str() {
                OID_SUBJECT_ALT_NAME => {
                    if let Ok((dns, ips)) = parse_subject_alt_name(extension) {
                        san_dns_names = dns;
                        san_ip_addresses = ips;
                    }
                }
                OID_BASIC_CONSTRAINTS => {
                    if let Ok(ca_flag) = parse_basic_constraints(extension) {
                        is_ca = ca_flag;
                    }
                }
                OID_KEY_USAGE => {
                    if let Ok(usages) = parse_key_usage(extension) {
                        key_usage = usages;
                    }
                }
                OID_AUTHORITY_INFO_ACCESS => {
                    if let Ok((ocsp, crl)) = parse_authority_info_access(extension) {
                        ocsp_urls = ocsp;
                        crl_urls = crl;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(ParsedCertificate {
        subject,
        issuer,
        san_dns_names,
        san_ip_addresses,
        is_ca,
        key_usage,
        not_before,
        not_after,
        serial_number,
        ocsp_urls,
        crl_urls,
        subject_der,
        public_key_der,
        key_algorithm,
        key_size,
    })
}

/// Parse certificate from PEM data to extract actual certificate information
pub fn parse_certificate_from_pem(pem_data: &str) -> Result<ParsedCertificate, TlsError> {
    // Parse PEM to get DER bytes using rustls-pemfile
    let mut cursor = std::io::Cursor::new(pem_data.as_bytes());
    let cert_der = rustls_pemfile::certs(&mut cursor)
        .next()
        .ok_or_else(|| TlsError::CertificateParsing("No certificate in PEM data".to_string()))?
        .map_err(|e| TlsError::CertificateParsing(format!("Failed to parse PEM: {e}")))?;

    // Parse X.509 certificate using x509-cert
    let cert = X509CertCert::from_der(&cert_der)
        .map_err(|e| TlsError::CertificateParsing(format!("X.509 parsing failed: {e}")))?;

    // Delegate to the DER function to avoid code duplication
    parse_x509_certificate_from_der_internal(&cert)
}

/// Parse certificate from DER data to extract actual certificate information
pub fn parse_certificate_from_der(der_bytes: &[u8]) -> Result<ParsedCertificate, TlsError> {
    // Parse X.509 certificate using x509-cert
    let cert = X509CertCert::from_der(der_bytes)
        .map_err(|e| TlsError::CertificateParsing(format!("X.509 parsing failed: {e}")))?;

    // Delegate to the internal function
    parse_x509_certificate_from_der_internal(&cert)
}
