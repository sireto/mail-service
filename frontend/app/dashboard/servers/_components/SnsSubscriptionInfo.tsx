"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Copy, CheckCircle } from "lucide-react";

export default function SnsSubscriptionInfo() {
  const snsUrl = "https://email.sireto.dev/api/bounce-logs/sns/bounce";
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(snsUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="w-full mx-auto p-6 bg-accent text-accent-foreground rounded-lg shadow-lg my-8">
      <h2 className="text-xl font-semibold mb-2">Subscribe to SNS Topic</h2>
      <p className="text-sm text-muted-foreground">
        To enable email status tracking via our email service, subscribe the following
        endpoint in your <strong>AWS SNS Topic</strong> settings.
      </p>

      <div className="flex items-center mt-3 bg-muted p-2 rounded-md">
        <code className="mr-4 text-sm break-all">{snsUrl}</code>
        <Button
          onClick={handleCopy}
          className="ml-2"
          size="sm"
          variant="outline"
        >
          {copied ? (
            <CheckCircle className="w-4 h-4 text-green-500" />
          ) : (
            <Copy className="w-4 h-4" />
          )}
        </Button>
      </div>

      <h3 className="mt-4 font-semibold text-sm">Steps to Subscribe:</h3>
      <ul className="text-sm list-disc pl-5 text-muted-foreground space-y-1">
        <li>Go to the <strong>AWS SNS Console</strong>.</li>
        <li>Select the SNS Topic you want to use.</li>
        <li>Click on <strong>Create Subscription</strong>.</li>
        <li>Choose <strong>HTTPS</strong> as the Protocol.</li>
        <li>Paste the copied URL in the Endpoint field.</li>
        <li>In the delivery policy section, Uncheck the <strong>Use the default delivery policy</strong> and Update the Content-Type to <strong>application/json</strong>.</li>
        <li>Click on <strong>Create Subscription</strong>.</li>
      </ul>
    </div>
  );
}
