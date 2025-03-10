export const runtime = 'nodejs';

import { NextRequest, NextResponse } from "next/server";

/**
 * @description A POST function to parse MJML to HTML
 * @returns Parsed HTML string or an error response
 */
export async function POST(req: NextRequest) {
    try {
        const { mjml_content } = await req.json();

        if (!mjml_content) {
            return NextResponse.json({ error: "MJML content is required" }, { status: 400 });
        }
        console.log("THE MJML content ===> ", mjml_content);
        
        // Dynamically import mjml2html at runtime
        const mjml = await import('mjml');
        const { html, errors } = mjml.default(mjml_content);

        if (errors && errors.length > 0) {
            console.error("MJML Parsing Errors:", errors);
            return NextResponse.json({ error: "MJML parsing failed", details: errors }, { status: 400 });
        }

        return NextResponse.json({ html }, { status: 200 });
    } catch (err) {
        console.error("Server Error:", err);
        return NextResponse.json({ error: "Internal Server Error", details: (err as Error).message }, { status: 500 });
    }
}
