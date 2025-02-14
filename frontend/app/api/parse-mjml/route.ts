import { NextRequest, NextResponse } from "next/server";
import mjml2html from "mjml";

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

        // console.log("Received MJML content:", mjml_content);

        console.warn("BEFORE THE PARSING");
        const { html } = mjml2html(mjml_content);
        console.warn("AFTER THE PARSING");


        // if (errors.length) {
        //     console.error("MJML Parsing Errors:", errors);
        //     return NextResponse.json({ error: "MJML parsing failed", details: errors }, { status: 400 });
        // }
        // const stringified_json = JSON.stringify({
        //     html
        // });

        return NextResponse.json({ data: html }, { status: 200 });
    } catch (err) {
        console.error("Server Error:", err);
        return NextResponse.json({ error: "Internal Server Error", details: (err as Error).message }, { status: 500 });
    }
}
