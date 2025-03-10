import { useState } from "react";
import { DialogTitle, DialogDescription, DialogHeader } from "@/components/ui/dialog";
import { Form, FormField, FormItem, FormLabel, FormControl, FormMessage } from "@/components/ui/form";
import { Textarea } from "@/components/ui/textarea";
import { AddTemplateFormSchemaDTO } from "@/lib/type";
import { Input } from "@/components/ui/input";
import { UseFormReturn } from "react-hook-form";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import { ScanEye } from "lucide-react";
import { usePreviewTemplateMutation } from "@/app/services/TemplateApi";
import Modal from "@/components/Modal";
import PreviewFrame from "./PreviewFrame";

interface TemplateModalBodyProps {
    modalTitle: string;
    modalDescription: string;
    form: UseFormReturn<z.infer<typeof AddTemplateFormSchemaDTO>>;
    submitHandler: (value: z.infer<typeof AddTemplateFormSchemaDTO>) => void;
    isUpdating: boolean;
    triggerButton: React.ReactNode;
}

const PreviewDialogTitle = ({ title }: { title: string }) => {
    return <DialogTitle>
        { title }
    </DialogTitle>
}

const TemplateModalBody = ({
    modalTitle,
    modalDescription,
    form,
    submitHandler,
    triggerButton,
}: TemplateModalBodyProps) => {
    const [previewTemplate, { isLoading: isPreviewing, error: previewError }] = usePreviewTemplateMutation();
    const [parsedHtml, setParsedHtml] = useState<string | null>(null);
    const previewHandler = async() => {
        const value = form.getValues();
        const resultHtml = await previewTemplate({
            mjml: value.raw_mjml_content.trim()
        });

        if (resultHtml && resultHtml.data) setParsedHtml(resultHtml?.data.html);
    };

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
                <Modal
                      triggerButton={
                          <Button variant={"default"} className='!mt-0' onClick={previewHandler} >
                              <ScanEye size={16} />
                              <span>Preview</span>
                          </Button>
                      }
                      dialogBody={<PreviewFrame html={parsedHtml || "Loading..."} modalTitle={form.getValues().name} modalDescription={"Preview your template"}/>}
                      dialogTitle={form.getValues().name}
                      dialogDescription={"Preview your template"}
                      classname="min-h-[80%]"
                  />
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
                                    placeholder="Template name" {...field} 
                                    className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                                />
                            </FormControl>
                            <FormMessage>{form.formState.errors.name?.message}</FormMessage>
                        </FormItem>
                    )}
                    />
                    <FormField
                        control={form.control}
                        name="raw_mjml_content"
                        render={({ field, fieldState }) => (
                            <FormItem>
                                <FormLabel className='font-bold text-black'>MJML Content</FormLabel>
                                <FormControl>
                                    <Textarea 
                                        rows={16} 
                                        placeholder="MJML content" 
                                        {...field} 
                                        className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                                    />
                                </FormControl>
                                <FormMessage>{form.formState.errors.raw_mjml_content?.message}</FormMessage>
                            </FormItem>
                        )}
                    />
                    { triggerButton }
                </form>
            </Form>
        </>
    )
}

export default TemplateModalBody;