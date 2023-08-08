import { IFile } from "@/types/file";
import axiosInstance, { API_ENDPOINTS } from "@/utils/axios";

export function getMeta(hash: string): Promise<IFile> {
    return axiosInstance.get(API_ENDPOINTS.file.meta(hash)).then((response) => response.data);
};

export function uploadFile(file: File): Promise<IFile> {
    const formData = new FormData();
    formData.append('file', file);
    return axiosInstance.post(API_ENDPOINTS.file.upload, formData).then((response) => response.data);
};

export function downloadFile(hash: string): Promise<Response> {
    return axiosInstance.get(API_ENDPOINTS.file.download(hash));
}