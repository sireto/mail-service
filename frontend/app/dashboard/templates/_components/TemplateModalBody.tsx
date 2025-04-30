import { useRef } from "react";
import { DialogTitle, DialogDescription, DialogHeader } from "@/components/ui/dialog";
import { Form, FormField, FormItem, FormLabel, FormControl, FormMessage } from "@/components/ui/form";
import { AddTemplateFormSchemaDTO } from "@/lib/type";
import { Input } from "@/components/ui/input";
import { UseFormReturn } from "react-hook-form";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import { ScanEye } from "lucide-react";
import Modal from "@/components/Modal";
import PreviewFrame from "./PreviewFrame";
import Editor from '@monaco-editor/react';
import type { editor } from 'monaco-editor';


interface TemplateModalBodyProps {
    modalTitle: string;
    modalDescription: string;
    form: UseFormReturn<z.infer<typeof AddTemplateFormSchemaDTO>>;
    submitHandler: (value: z.infer<typeof AddTemplateFormSchemaDTO>) => void;
    isUpdating: boolean;
    triggerButton: React.ReactNode;
}

const TemplateModalBody = ({
    modalTitle,
    modalDescription,
    form,
    submitHandler,
    triggerButton,
}: TemplateModalBodyProps) => {
    const value = form.getValues();
    const mjml = value.raw_mjml_content.trim();
    
    const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);

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
                          <Button variant={"default"} className='!mt-0'  >
                              <ScanEye size={16} />
                              <span>Preview</span>
                          </Button>
                      }
                      dialogBody={<PreviewFrame mjml={ mjml } modalTitle={form.getValues().name} modalDescription={"Preview your template"}/>}
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
                                <FormLabel className='font-bold text-black w-full flex items-center justify-between'>
                                    <span>MJML Content</span>
                                    <Button 
                                        variant={'outline'}
                                        type="button"
                                        onClick={() => editorRef.current?.getAction('editor.action.formatDocument')?.run() }
                                    >
                                        Format
                                    </Button>
                                </FormLabel>
                                <FormControl className="relative">
                                    <Editor
                                        height={400}
                                        defaultLanguage="xml"
                                        value={field.value}
                                        options={{
                                            wordWrap: 'on',
                                            lineNumbers: 'off',
                                            minimap: { enabled: false },
                                        }}
                                        onMount={(editor) => {
                                            editorRef.current = editor;
                                        }}
                                        onChange={(value) => field.onChange(value)}
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