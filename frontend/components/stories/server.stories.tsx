/* eslint-disable @typescript-eslint/no-explicit-any */
import { Meta, StoryObj } from "@storybook/react";
import ServerCard from "@/components/ServerCard";
import { ReduxProvider } from "@/providers/providers";

const validTlsTypes = ["NONE", "SSL/TLS", "STARTTLS"] as const;
const tlsTypeFromApi: string = "STARTTLS"; // Assume this comes dynamically

const mockServer = {
  id: "1",
  host: "smtp.example.com",
  port: 25,
  smtp_username: "username",
  smtp_password: "password",
  tls_type: validTlsTypes.includes(tlsTypeFromApi as any)
    ? (tlsTypeFromApi as "NONE" | "SSL/TLS" | "STARTTLS")
    : "NONE", // Default fallback
  namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
};

const meta: Meta<typeof ServerCard> = {
  title: "Components/ServerCard",
  component: ServerCard,
  tags: ["autodocs"],
  parameters: {
    layout: "fullscreen",
  },
  decorators: [
    (Story) => (
      <ReduxProvider>
        <Story />
      </ReduxProvider>
    ),
  ],
};

export default meta;
type Story = StoryObj<typeof ServerCard>;

// Default Story
export const Default: Story = {
  args: {
    server: mockServer,
  },
};
