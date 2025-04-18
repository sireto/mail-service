import React, { ReactNode } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";

interface CampaignSenderDialogProps {
  open?: boolean;
  onClose: () => void;
  title: string;
  description: string;
  children: ReactNode;
  isSubmitting: boolean;
  onSubmit: () => void;
  submitButtonText?: string;
}

const CampaignSenderDialog: React.FC<CampaignSenderDialogProps> = ({
  open,
  onClose,
  title,
  description,
  children,
  isSubmitting,
  onSubmit,
  submitButtonText = "Save",
}) => {
  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle className="text-xl font-medium">{title}</DialogTitle>
          <DialogDescription className="text-sm text-muted-foreground mt-1">
            {description}
          </DialogDescription>
        </DialogHeader>

        {children}

        <DialogFooter>
          <Button type="button" variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button
            type="submit"
            disabled={isSubmitting}
            className="bg-blue-600 hover:bg-blue-700"
            onClick={onSubmit}
          >
            {isSubmitting ? "Processing..." : submitButtonText}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export { CampaignSenderDialog };
