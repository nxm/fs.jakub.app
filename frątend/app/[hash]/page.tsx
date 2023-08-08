import { getMeta } from "@/sections/file/file-api";
import FileView from "@/sections/file/file-view";

export default async function Page(
    { params }: { params: { hash: string } }
) {
    const { hash } = params;

    const file = await getMeta(hash)
    .catch((err) => {
        console.error(err);
        return null;
    });
    
    return <FileView file={file} />;

}