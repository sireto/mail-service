import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

interface ToolTipProps {
  message: string;
  tooltipTrigger: React.ReactNode;
}

const ToolTip = ({ message, tooltipTrigger }: ToolTipProps) => {
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger>{tooltipTrigger}</TooltipTrigger>
        <TooltipContent>
          <p>{message}</p>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
};

export default ToolTip;
