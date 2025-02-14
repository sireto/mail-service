'use client';

import React from 'react';
import {
    ColumnDef,
    flexRender,
    getCoreRowModel,
    useReactTable,
} from "@tanstack/react-table";
import { Table, TableHeader, TableRow, TableHead, TableBody, TableCell } from './ui/table';

interface DataTableProps<TData, TValue> {
    data: TData[]
    columns: ColumnDef<TData, TValue>[],
    fallback: String
};

/**
 * 
 * @param param0 
 * @description create a data table component with any number of columns...
 * @returns 
 */
export function DataTable<TData, TValue>({
    data,
    columns,
    fallback
}: DataTableProps<TData, TValue>) {
    const table = useReactTable({
        data,
        columns,
        getCoreRowModel: getCoreRowModel(),
    });

    return (
            <Table>
                <TableHeader className='bg-secondary'>
                    {table.getHeaderGroups().map((headerGroup) => (
                        <TableRow key={headerGroup.id}>
                            {headerGroup.headers.map((header) => (
                                <TableHead key={header.id} className='font-bold'>
                                    {header.isPlaceholder
                                        ? null
                                        : flexRender(
                                              header.column.columnDef.header,
                                              header.getContext()
                                          )}
                                </TableHead>
                            ))}
                        </TableRow>
                    ))}
                </TableHeader>
                <TableBody>
                    {table.getRowModel().rows?.length ? (
                        table.getRowModel().rows.map((row) => (
                            <TableRow key={row.id} >
                                {row.getVisibleCells().map((cell, idx) => (
                                    <TableCell key={cell.id} className={`${idx === 0 ? 'text-primary' : 'text-secondary-foreground/60'} min-w-[80px]`}>
                                        {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                    </TableCell>
                                ))}
                            </TableRow>
                        ))
                    ) : (
                        <TableRow>
                            <TableCell colSpan={columns.length} className="h-24 text-center">
                                { fallback }
                            </TableCell>
                        </TableRow>
                    )}
                </TableBody>
            </Table>
    );
}

export default DataTable;
