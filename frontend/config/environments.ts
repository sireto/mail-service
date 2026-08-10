const environments = {
  API_BASE_URL: process.env.NEXT_PUBLIC_BASE_URL,
  TEMPLATE_API_BASE_URL: `${process.env.NEXT_PUBLIC_BASE_URL}/templates`,
  LIST_API_BASE_URL: `${process.env.NEXT_PUBLIC_BASE_URL}/list`,
  CONTACT_API_BASE_URL: `${process.env.NEXT_PUBLIC_BASE_URL}/contacts`,
  CAMPAIGN_API_BASE_URL: `${process.env.NEXT_PUBLIC_BASE_URL}/campaigns`,
  MAIL_API_BASE_URL: `${process.env.NEXT_PUBLIC_BASE_URL}/mails`,
  CAMPAIGN_SENDER_API_BASE_URL: `${process.env.NEXT_PUBLIC_BASE_URL}/campaign-senders`,
};

export default environments;
