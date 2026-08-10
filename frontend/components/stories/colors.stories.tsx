import type { Meta, StoryObj } from "@storybook/react";

const Colors = () => (
  <section>
    <h1>Colors</h1>
    <p>Colors used</p>
    <dl>
      <dt>White</dt>
      <dd>#ffffff</dd>
      <dt>Primary</dt>
      <dd>#0070d6</dd>
    </dl>
  </section>
);

const meta = {
  title: "App/Colors",
  component: Colors,
} satisfies Meta<typeof Colors>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Palette: Story = {};
