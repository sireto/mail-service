import type { Meta, StoryObj } from "@storybook/react";
import Modal from "@/components/Modal";
import ReduxProvider from "@/providers/redux-provider";
import { Button } from "../ui/button";
import { Plus } from "lucide-react";
import { DialogFooter, DialogClose } from "../ui/dialog";

const MockDialogBody = <div>
    <div>Modal content</div>
    <DialogFooter>
      <DialogClose className='mt-2 md:mt-0' asChild>
          <Button type="button" variant={"outline"}>Close</Button>
      </DialogClose>
      <Button type="submit" >Create</Button>
    </DialogFooter>
</div>;

const mockModal = {
  triggerBody: "Story modal",
  dialogBody: MockDialogBody,
  triggerButton: <Button variant={"default"}>
          <>
              <Plus size={24} />
              <span className='ml-1'>Open Modal</span>
          </>
      </Button>,
  dialogTitle: "Story modal title",
  dialogDescription: "Story modal description",
}

const meta: Meta<typeof Modal> = {
  title: "Components/Modal",
  component: Modal,
  tags: ["autodocs"],
  parameters: {
    layout: "fullscreen",
  },
  decorators: [
    (Story) => (
      <ReduxProvider>
        <Story />
      </ReduxProvider>
    )
  ]
};

export default meta;
type Story = StoryObj<typeof Modal>;

export const Default: Story = {
  args: {
    // triggerBody: mockModal.triggerBody, // Removed as it is not part of ModalProps
    dialogBody: mockModal.dialogBody,
    triggerButton: mockModal.triggerButton,
    dialogTitle: mockModal.dialogTitle,
    dialogDescription: mockModal.dialogDescription
  }
};
