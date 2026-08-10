import { z } from "zod";
import { ListDTO } from "@/lib/type";

export type List = z.infer<typeof ListDTO>;
