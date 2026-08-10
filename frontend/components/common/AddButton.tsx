import { Plus } from "lucide-react";
import { Button } from "../ui/button";

const AddButton = (props: React.ComponentProps<typeof Button>) => (
  <Button {...props} variant={"default"}>
    <>
      <Plus size={24} />
      <span className="ml-1">New</span>
    </>
  </Button>
);

export default AddButton;
