"use client";

import React from "react";
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "./ui/table";
import { Button } from "./ui/button";
import { ChevronLeft, ChevronRight } from "lucide-react";
import type { Page } from "@/lib/type/pagination";

/**
 * Server-side paging state. The table renders whatever page it is given and reports which
 * page is wanted next; it never slices client-side, because the rows beyond this page were
 * never sent.
 */
export interface DataTablePagination {
  page: Pick<Page<unknown>, "total" | "limit" | "offset" | "has_more">;
  onOffsetChange: (offset: number) => void;
}

interface DataTableProps<TData, TValue> {
  data: TData[];
  columns: ColumnDef<TData, TValue>[];
  fallback: string;
  isLoading?: boolean;
  /** Omit to render an unpaginated table. */
  pagination?: DataTablePagination;
}

/**
 *
 * @param param0
 * @description create a data table component with any number of columns...
 * @returns
 */
export function DataTable<TData, TValue>({
  data,
  columns,
  fallback,
  isLoading = false,
  pagination,
}: DataTableProps<TData, TValue>) {
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    // Paging is server-side; react-table is told the row count so it does not assume the
    // data it holds is the whole set.
    manualPagination: true,
    rowCount: pagination?.page.total,
  });

  const { page, onOffsetChange } = pagination ?? {};
  const rangeStart = page && page.total > 0 ? page.offset + 1 : 0;
  const rangeEnd = page ? page.offset + data.length : 0;
  const canGoBack = !!page && page.offset > 0;
  const canGoForward = !!page && page.has_more;

  return (
    <>
    <Table>
      <TableHeader className="bg-secondary">
        {table.getHeaderGroups().map((headerGroup) => (
          <TableRow key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <TableHead key={header.id} className="font-bold">
                {header.isPlaceholder
                  ? null
                  : flexRender(
                      header.column.columnDef.header,
                      header.getContext(),
                    )}
              </TableHead>
            ))}
          </TableRow>
        ))}
      </TableHeader>
      <TableBody>
        {!isLoading ? (
          table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableRow key={row.id}>
                {row.getVisibleCells().map((cell, idx) => (
                  <TableCell
                    key={cell.id}
                    className={`${
                      idx === 0
                        ? "text-primary"
                        : "text-secondary-foreground/60"
                    }`}
                  >
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell colSpan={columns.length} className="h-24 text-center">
                {fallback}
              </TableCell>
            </TableRow>
          )
        ) : (
          <TableRow>
            <TableCell colSpan={columns.length} className="h-24 text-center">
              Loading...
            </TableCell>
          </TableRow>
        )}
      </TableBody>
    </Table>

      {page && page.total > 0 && (
        <div className="flex items-center justify-between gap-4 px-2 py-3 text-sm text-secondary-foreground/70">
          <span>
            Showing {rangeStart.toLocaleString()}&ndash;{rangeEnd.toLocaleString()} of{" "}
            {page.total.toLocaleString()}
          </span>
          <div className="flex items-center gap-2">
            <Button
              type="button"
              variant="outline"
              size="sm"
              disabled={!canGoBack || isLoading}
              onClick={() =>
                onOffsetChange?.(Math.max(0, page.offset - page.limit))
              }
              aria-label="Previous page"
            >
              <ChevronLeft className="h-4 w-4" />
              Previous
            </Button>
            <Button
              type="button"
              variant="outline"
              size="sm"
              disabled={!canGoForward || isLoading}
              onClick={() => onOffsetChange?.(page.offset + page.limit)}
              aria-label="Next page"
            >
              Next
              <ChevronRight className="h-4 w-4" />
            </Button>
          </div>
        </div>
      )}
    </>
  );
}

export default DataTable;
