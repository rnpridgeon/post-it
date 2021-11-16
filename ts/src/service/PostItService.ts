import axios, { AxiosResponse } from "axios";

const apiClient = axios.create({
  baseURL: "/api",
  headers: {
    Accept: "application/json",
    "Content-Type": "application/json",
  },
});

export default {
  listMessage(): Promise<AxiosResponse<Array<string>, Error>> {
    return apiClient.get("/message");
  },

  postMessage(msg_data: string): Promise<AxiosResponse<string, Error>> {
    return apiClient.post("/message", { content: msg_data });
  },
};
