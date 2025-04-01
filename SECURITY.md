# Security Policy

## Reporting a Vulnerability

We take the security of mdserve-examples seriously. If you believe you have found a security vulnerability, please follow these steps:

1. **Do not** disclose the vulnerability publicly
2. Send details of the vulnerability to [security@example.com](mailto:security@example.com)
3. Include the following in your report:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any suggestions for remediation

## Response Timeline

We strive to respond to security reports promptly:

- **Initial response**: Within 48 hours
- **Vulnerability assessment**: Within 1 week
- **Remediation plan**: Within 2 weeks (depending on severity)

## Security Best Practices

When using mdserve for your documentation:

### Content Security

- Avoid including sensitive information in markdown files
- Be cautious with embedded scripts or iframes
- Validate any user-contributed content before publishing

### Server Configuration

- Keep mdserve updated to the latest version
- Use HTTPS when deploying to production
- Configure proper access controls for your content

### Template Security

- Audit custom templates for security vulnerabilities
- Sanitize any dynamic content that gets rendered
- Avoid using unsafe inline scripts

## Version Information

This security policy was last updated on April 1, 2025.

---

For general questions about mdserve security, please visit our [documentation](https://github.com/intellicode/mdserve).
