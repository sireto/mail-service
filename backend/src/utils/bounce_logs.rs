use crate::models::bounce_logs::Message;

pub fn search_header_from_header_list(message: &Message, header_name: &str) -> Option<String> {
    let message_id = message
        .mail
        .headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case(header_name))
        .map(|header| header.value.clone());

    message_id
}

/// Hosts we are willing to issue an SNS subscription-confirmation request to.
///
/// The confirmation handler performs an outbound GET to a URL taken straight out of an
/// unauthenticated request body. Without this check that is a server-side request forgery
/// primitive: any caller could make the service fetch cloud instance metadata or an
/// internal admin endpoint. Amazon always serves SubscribeURL from sns.<region>.amazonaws.com
/// (or the China partition), so anything else is either a mistake or an attack.
pub fn is_trusted_sns_url(raw_url: &str) -> bool {
    let Ok(parsed) = url::Url::parse(raw_url) else {
        return false;
    };

    if parsed.scheme() != "https" {
        return false;
    }

    let Some(host) = parsed.host_str() else {
        return false;
    };
    let host = host.to_ascii_lowercase();

    // Guard against a host like "sns.amazonaws.com.attacker.tld" by matching on suffix
    // boundaries rather than a bare `contains`.
    let trusted_suffixes = [".amazonaws.com", ".amazonaws.com.cn"];
    if !trusted_suffixes.iter().any(|suffix| host.ends_with(suffix)) {
        return false;
    }

    host.starts_with("sns.")
}

#[cfg(test)]
mod tests {
    use super::is_trusted_sns_url;

    #[test]
    fn accepts_real_sns_endpoints() {
        assert!(is_trusted_sns_url(
            "https://sns.us-east-1.amazonaws.com/?Action=ConfirmSubscription&Token=abc"
        ));
        assert!(is_trusted_sns_url("https://sns.cn-north-1.amazonaws.com.cn/?Token=abc"));
    }

    #[test]
    fn rejects_ssrf_attempts() {
        // The metadata endpoint, the classic target.
        assert!(!is_trusted_sns_url("http://169.254.169.254/latest/meta-data/"));
        // Plain HTTP, even on a real SNS host.
        assert!(!is_trusted_sns_url("http://sns.us-east-1.amazonaws.com/"));
        // Suffix smuggling.
        assert!(!is_trusted_sns_url("https://sns.amazonaws.com.attacker.tld/"));
        // Right domain, wrong service.
        assert!(!is_trusted_sns_url("https://s3.us-east-1.amazonaws.com/"));
        // Userinfo trick.
        assert!(!is_trusted_sns_url("https://sns.us-east-1.amazonaws.com@attacker.tld/"));
        assert!(!is_trusted_sns_url("not a url"));
        assert!(!is_trusted_sns_url(""));
    }
}
