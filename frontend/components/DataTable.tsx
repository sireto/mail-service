'use client';

import React from 'react';
import {
    ColumnDef,
    flexRender,
    getCoreRowModel,
    useReactTable,
} from "@tanstack/react-table";
import { Table, TableCaption, TableHeader, TableRow, TableHead, TableBody, TableCell } from './ui/table';

interface DataTableProps<TData, TValue> {
    data: TData[]
    columns: ColumnDef<TData, TValue>[]
};

/**
 * 
 * @param param0 
 * @description create a data table component with any number of columns...
 * @returns 
 */
export function DataTable<TData, TValue>({
    data,
    columns
}: DataTableProps<TData, TValue>) {
    const table = useReactTable({
        data,
        columns,
        getCoreRowModel: getCoreRowModel(),
    });

    return (
            <Table>
                <TableCaption>A list of templates</TableCaption>
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
                            <TableRow key={row.id} className='cursor-pointer'>
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
                                No templates.
                            </TableCell>
                        </TableRow>
                    )}
                </TableBody>
            </Table>
    );
}

export default DataTable;
