import { z } from "zod";

/**
 * The API's list envelope.
 *
 * The list endpoints used to return a bare array of every row in the table. They now return
 * one page plus the total, which is what lets the UI say "51-100 of 4,312" instead of
 * silently showing whatever happened to arrive.
 */
export const PageSchema = <T extends z.ZodTypeAny>(item: T) =>
  z.object({
    items: z.array(item),
    total: z.number(),
    limit: z.number(),
    offset: z.number(),
    has_more: z.boolean(),
  });

export interface Page<T> {
  items: T[];
  total: number;
  limit: number;
  offset: number;
  has_more: boolean;
}

/** Matches DEFAULT_PAGE_SIZE in backend/src/models/pagination.rs. */
export const DEFAULT_PAGE_SIZE = 50;

/** Matches MAX_PAGE_SIZE. Asking for more is clamped server-side, not rejected. */
export const MAX_PAGE_SIZE = 200;

export interface PageParams {
  limit?: number;
  offset?: number;
}

/** Serialise page params onto a query string, leaving defaults to the server. */
export const appendPageParams = (
  params: URLSearchParams,
  page?: PageParams,
): URLSearchParams => {
  if (page?.limit !== undefined) params.append("limit", String(page.limit));
  if (page?.offset !== undefined) params.append("offset", String(page.offset));
  return params;
};

/** An empty page, for rendering before the first response arrives. */
export const emptyPage = <T>(limit = DEFAULT_PAGE_SIZE): Page<T> => ({
  items: [],
  total: 0,
  limit,
  offset: 0,
  has_more: false,
});
