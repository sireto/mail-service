/* eslint-disable @typescript-eslint/no-explicit-any */
import { Meta, StoryObj } from "@storybook/react";
import ServerCard from "@/components/ServerCard";

// Define mock functions for onUpdate, onDelete, and onCreate
const mockUpdate = async (id: string, data: any) => {
  console.log("Updating server with id:", id, data);
};

const mockDelete = async (id: string) => {
  console.log("Deleting server with id:", id);
};

const mockCreate = async (data: any) => {
  console.log("Creating new server:", data);
};

// Updated mockServer with correct tls_type
const mockServer = {
  id: "1",
  host: "smtp.example.com",
  port: 25,
  smtp_username: "username",
  smtp_password: "password",
  tls_type: "STARTTLS",
  namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
};

const meta: Meta<typeof ServerCard> = {
  title: "Components/ServerCard",
  component: ServerCard,
  tags: ["autodocs"],
  parameters: {
    layout: "fullscreen",
  },
};

export default meta;
type Story = StoryObj<typeof ServerCard>;

// Default Story
export const Default: Story = {
  args: {
    server: mockServer,
    onUpdate: mockUpdate,
    onDelete: mockDelete,
    onCreate: mockCreate,
  },
};
