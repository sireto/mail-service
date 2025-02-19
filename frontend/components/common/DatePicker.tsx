import { cn } from '@/lib/utils';
import { Popover, PopoverTrigger, PopoverContent } from '@radix-ui/react-popover';
import { CalendarIcon } from 'lucide-react';
import React from 'react';
import { Button } from '@/components/ui/button';
import { FormField, FormItem, FormLabel, FormControl, FormMessage } from '../ui/form';
import { format } from 'date-fns';
import { Calendar } from '../ui/calendar';
import { UseFormReturn } from 'react-hook-form';

interface DatePickerProps {
    form: UseFormReturn<any>,
    label: string,
    fieldName: string,
    placeholder: string,
}

const DatePicker = ({
    form,
    label,
    fieldName,
    placeholder,
}: DatePickerProps) => {
  return (
    <FormField
    control={form.control}
        name={fieldName}
        render={({ field }) => (
            <FormItem className="flex flex-col w-full">
            <FormLabel>{label}</FormLabel>
            <Popover>
                <PopoverTrigger asChild>
                <FormControl>
                    <Button
                    variant={"outline"}
                    className={cn(
                        "w-full pl-3 text-left font-normal lg:min-w-[180px]",
                        !field.value && "text-muted-foreground"
                    )}
                    >
                    {field.value ? (
                        format(field.value, "PPP")
                    ) : (
                        <span>{placeholder}</span>
                    )}
                    <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                    </Button>
                </FormControl>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0 z-50" align="start">
                <Calendar
                    mode="single"
                    className='bg-white border border-secondary rounded-md' 
                    selected={field.value ? new Date(field.value) : undefined}
                    onSelect={field.onChange}
                    disabled={(date) =>
                    date > new Date() || date < new Date("1900-01-01")
                    }
                    initialFocus
                />
                </PopoverContent>
            </Popover>
            <FormMessage>{form.formState.errors.to?.message?.toString()}</FormMessage>
            </FormItem>
    )}
/>
  )
}

export default DatePicker