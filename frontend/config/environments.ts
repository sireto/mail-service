const environments = {
    API_BASE_URL: process.env.NEXT_PUBLIC_BASE_URL,
    TEMPLATE_API_BASE_URL: `${process.env.NEXT_PUBLIC_BASE_URL}/templates`,
    LIST_API_BASE_URL: `${process.env.NEXT_PUBLIC_BASE_URL}/list`,
};

export default environments;