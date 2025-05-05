import { TemplateDTO } from '@/lib/type';
import { z } from 'zod';

export type Template = z.infer<typeof TemplateDTO>;