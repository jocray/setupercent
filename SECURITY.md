# Security policy

## Supported versions

Only the latest release receives security updates.

| Version | Supported |
| --- | --- |
| Latest | Yes |
| Older | No |

## Reporting a vulnerability

Please do not open a public issue for security problems.

If you believe you have found a security vulnerability in setupercent, report it privately:

- Open a [private security advisory](https://github.com/jocray/setupercent/security/advisories/new) on GitHub, or
- Contact the maintainer directly (see the email on the GitHub profile).

Please include:

- A description of the issue and how to reproduce it
- The affected setupercent version and Windows build
- Any proof-of-concept code or screenshots, if available

## What to expect

- Acknowledgement within a few days
- An assessment of severity and impact
- A fix, if warranted, released as soon as is practical
- Public credit if you'd like it, once the fix is out

## Scope

setupercent is a local Windows utility. It reads device battery levels through the standard SetupAPI, stores its config in an NTFS alternate data stream of its own executable (with a fallback to `%APPDATA%\setupercent\config.json` on file systems without ADS support), and does not talk to the network. Vulnerabilities relevant to that threat model are in scope — for example, unsafe handling of config files, privilege escalation via the `.exe`, or crashes triggered by crafted device data.