    pub async fn create_connection(
        &self,
        host: &str,
        port: u16,
    ) -> Result<tokio_rustls::client::TlsStream<TcpStream>, TlsError> {
        tracing::debug!("Creating enterprise TLS connection to {}:{}", host, port);
        
        // Create TCP connection with timeout
        tracing::debug!("TLS: About to create TCP connection to {}:{}", host, port);
        tracing::debug!("TLS: Resolving DNS for {}", host);
        
        // First try to resolve the address to see if DNS is the issue
        let addr = format!("{}:{}", host, port);
        tracing::debug!("TLS: About to resolve address: {}", addr);
        
        // Simple timeout test - if this hangs, the issue is with the async runtime
        tracing::debug!("TLS: Testing if timeout works at all");
        let timeout_test = tokio::time::timeout(
            Duration::from_millis(100),
            async { tokio::time::sleep(Duration::from_millis(200)).await }
        ).await;
        
        match timeout_test {
            Ok(_) => tracing::error!("TLS: TIMEOUT NOT WORKING - sleep completed when it should have timed out!"),
            Err(_) => tracing::debug!("TLS: Timeout is working correctly"),
        }
        
        // Now try the real connection with a short timeout
        tracing::debug!("TLS: Starting actual TCP connect to {}", addr);
        let tcp_stream = tokio::time::timeout(
            Duration::from_secs(3),
            TcpStream::connect(&addr)
        ).await
            .map_err(|_| {
                tracing::error!("TLS: TCP connection timeout to {} after 3 seconds", addr);
                TlsError::Internal("Connection timeout".to_string())
            })?
            .map_err(|e| {
                tracing::error!("TLS: TCP connection failed to {}: {}", addr, e);
                TlsError::Internal(format!("Failed to connect to {addr}: {e}"))
            })?;
        tracing::debug!("TLS: TCP connection established to {}:{}", host, port);

        // Create enterprise TLS client configuration
        tracing::debug!("TLS: About to create client config");
        let client_config = self.create_client_config_sync()?;
        tracing::debug!("TLS: Client config created successfully");
        
        // Create TLS connector
        tracing::debug!("TLS: About to create TLS connector");
        let connector = TlsConnector::from(Arc::new(client_config));
        tracing::debug!("TLS: TLS connector created successfully");
        
        // Create server name for TLS
        tracing::debug!("TLS: About to create server name for {}", host);
        let server_name = rustls::pki_types::ServerName::try_from(host.to_string())
            .map_err(|e| {
                tracing::error!("TLS: Invalid hostname '{}': {}", host, e);
                TlsError::Internal(format!("Invalid hostname '{host}': {e}"))
            })?;
        tracing::debug!("TLS: Server name created successfully for {}", host);

        // Perform TLS handshake
        tracing::debug!("TLS: About to perform TLS handshake with {}", host);
        let tls_stream = connector.connect(server_name, tcp_stream).await
            .map_err(|e| {
                tracing::error!("TLS: TLS handshake failed with {}: {}", host, e);
                TlsError::Internal(format!("TLS handshake failed: {e}"))
            })?;
        tracing::debug!("TLS: TLS handshake completed successfully with {}", host);

        tracing::info!("Enterprise TLS connection established to {}:{}", host, port);
        Ok(tls_stream)
    }