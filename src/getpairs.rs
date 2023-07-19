pub fn get_pairs() -> Vec<(i32, &'static str, char)> {
    vec![
        (20, "FTP data", 'T'),
        (21, "FTP control", 'T'),
        (22, "SSH", 'T'),
        (23, "Telnet", 'T'),
        (25, "SMTP", 'T'),
        (53, "DNS", 'T'),
        (67, "DHCP Server", 'U'),
        (68, "DHCP Client", 'T'),
        (80, "HTTP", 'T'),
        (110, "POP3", 'T'),
        (123, "NTP", 'U'),
        (137, "NetBIOS", 'U'),
        (138, "NetBIOS", 'U'),
        (139, "NetBIOS", 'T'),
        (143, "IMAP4", 'T'),
        (161, "SNMP", 'U'),
        (389, "LDAP", 'T'),
        (443, "HTTPS", 'T'),
        (445, "SMB", 'T'),
        (548, "AFP", 'T'),
        (3389, "RDP", 'T'),
        (5900, "VNC", 'T'),
    ]
}
