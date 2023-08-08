'use client';

import { IFile } from "@/types/file";
import { useEffect, useState } from "react";
import { downloadFile } from "./file-api";

type Props = {
    file: IFile | null;
}

export default function FileView(props: Props) {
    const { file } = props;

    if (!file) {
        return (
            <main className="flex min-h-screen flex-col items-center p-24">
                <h1 className="text-4xl font-bold text-center">
                    fs.
                    <a className="text-blue-600" href="https://nextjs.org">
                        jakub.app
                    </a>
                </h1>
            </main>
        )
    }

    return (
        <main className="flex min-h-screen flex-col items-center p-8">
            <h1 className="text-4xl font-bold text-center">
                fs.
                <a className="text-blue-600" href="https://nextjs.org">
                    jakub.app
                </a>
            </h1>

            <div
                className="mt-4 flex max-w-fit border-blue-600 border-2 rounded-md"
            >

                {file?.content_type?.startsWith('image') && (
                    <img
                        className="object-contain w-fit"
                        src={`http://127.0.0.1:8080/download/${file.hash}`}
                        alt={file.name}
                    />
                )}

            </div>
        </main>
    );
}