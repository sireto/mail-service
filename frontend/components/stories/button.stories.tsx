import type { Meta, StoryObj } from "@storybook/react";

import { Button } from "@/components/ui/button";

import { action } from "@storybook/addon-actions";

const meta: Meta<typeof Button> = {
  title: "Components/ui/button",
  component: Button,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
  },
  argTypes: {
    variant: {
      control: "select",
      description: "Button Variants",
      options: [
        "default",
        "destructive",
        "outline",
        "secondary",
        "ghost",
        "link",
      ],
    },
    size: {
      control: "select",
      description: "Button sizes",
      options: ["default", "sm", "lg", "icon"],
    },
    disabled: {
      control: "boolean",
    },
    onClick: {
      action: "clicked",
      description: "Function called when the default button is clicked",
    },
    children: {
      control: "text",
      description: "Content to be displayed inside the button",
    },
    className: {
      control: "text",
      description: "Custom tailwind CSS classes to apply to the button",
    },
  },
};

export default meta;
type Story = StoryObj<typeof Button>;

export const Default: Story = {
  args: {
    variant: "default",
    size: "sm",
    disabled: false,
    onClick: action("default click"),
    children: "Default Button",
    className: "shadow-lg",
  },
};

export const Destructive: Story = {
  args: {
    variant: "destructive",
    size: "default",
    disabled: false,
    onClick: action("destructive click"),
    children: "Destructive Button",
    className: "shadow-lg",
  },
};

export const Outline: Story = {
  args: {
    variant: "outline",
    size: "default",
    disabled: false,
    onClick: action("outline click"),
    children: "Outline Button",
    className: "shadow-lg",
  },
};

export const Secondary: Story = {
  args: {
    variant: "secondary",
    size: "default",
    disabled: false,
    onClick: action("secondary click"),
    children: "Secondary Button",
    className: "shadow-lg",
  },
};

export const Ghost: Story = {
  args: {
    variant: "ghost",
    size: "default",
    disabled: false,
    onClick: action("ghost click"),
    children: "Ghost Button",
    className: "shadow-lg",
  },
};

export const Link: Story = {
  args: {
    variant: "link",
    size: "default",
    disabled: false,
    onClick: action("link click"),
    children: "Link Button",
    className: "shadow-lg",
  },
};

export const Small: Story = {
  args: {
    variant: "default",
    size: "sm",
    disabled: false,
    onClick: action("small button click"),
    children: "Small Button",
    className: "shadow-lg",
  },
};

export const Large: Story = {
  args: {
    variant: "default",
    size: "lg",
    disabled: false,
    onClick: action("large button click"),
    children: "Large Button",
    className: "shadow-lg",
  },
};

export const Disabled: Story = {
  args: {
    variant: "default",
    size: "default",
    disabled: true,
    onClick: action("disabled button click"),
    children: "Disabled Button",
    className: "shadow-lg",
  },
};
