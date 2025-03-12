import { SelectItem } from "@/components/ui/select";

type DropdownItem = {
  id: string;
  name: string;
};

type DropdownItemListProps<Item extends DropdownItem> = {
  items?: Item[];
};

function DropdownItemList<T extends DropdownItem>({
  items,
}: DropdownItemListProps<T>) {
  if (!items) return null;

  return items?.map((item) => (
    <SelectItem key={item.id} value={item.id}>
      {item.name}
    </SelectItem>
  ));
}

export default DropdownItemList;
