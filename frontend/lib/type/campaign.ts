import { z } from "zod";
import { CampaignDTO } from "../type";

export type Campaign = z.infer<typeof CampaignDTO>;
