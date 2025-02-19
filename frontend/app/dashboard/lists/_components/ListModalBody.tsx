import { DialogTitle, DialogDescription, DialogHeader } from "@/components/ui/dialog";
import { Form, FormField, FormItem, FormLabel, FormControl, FormMessage } from "@/components/ui/form";
import { Textarea } from "@/components/ui/textarea";
import { AddListFormSchemaDTO } from "@/lib/type";
import { Input } from "@/components/ui/input";
import { UseFormReturn } from "react-hook-form";
import { z } from "zod";

interface ListModalBodyProps {
    modalTitle: string;
    modalDescription: string;
    form: UseFormReturn<z.infer<typeof AddListFormSchemaDTO>>;
    submitHandler: (value: z.infer<typeof AddListFormSchemaDTO>) => void;
    triggerButton: React.ReactNode;
}

const ListModalBody = ({
    modalTitle,
    modalDescription,
    form,
    submitHandler,
    triggerButton,
}: ListModalBodyProps) => {
    return (
        <>
            <DialogHeader className='flex flex-row justify-between items-center'>
                      <div>
                        <DialogTitle>
                          {modalTitle}
                        </DialogTitle>
                        <DialogDescription className='mt-1'>
                          {modalDescription}
                        </DialogDescription>
                      </div>
                    </DialogHeader>
            <hr className='my-4'/>
    
            <Form {...form}>    {/* pass on the all the form-related methods allowing child components to access the form's context... */}
                <form 
                    onSubmit={form.handleSubmit(submitHandler)}
                    className='flex flex-col space-y-4'
                >
                    <FormField
                    control={form.control}
                    name="name"
                    render={({ field, fieldState }) => (
                        <FormItem>
                            <FormLabel className='font-bold text-black'>Name</FormLabel>
                            <FormControl>
                                <Input 
                                    placeholder="List name" 
                                    {...field} 
                                    className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                                />
                            </FormControl>
                            <FormMessage>{form.formState.errors.name?.message}</FormMessage>
                        </FormItem>
                    )}
                    />
                    <FormField
                        control={form.control}
                        name="description"
                        render={({ field, fieldState }) => (
                            <FormItem>
                                <FormLabel className='font-bold text-black'>Description</FormLabel>
                                <FormControl>
                                    <Textarea 
                                        rows={8} 
                                        placeholder="Add Description here..." 
                                        {...field} 
                                        className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                                    />
                                </FormControl>
                                <FormMessage>{form.formState.errors.description?.message}</FormMessage>
                            </FormItem>
                        )}
                    />
                    { triggerButton }
                </form>
            </Form>
        </>
    )
}

export default ListModalBody;