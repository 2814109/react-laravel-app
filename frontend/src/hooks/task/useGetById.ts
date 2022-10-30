import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
const endpoint = import.meta.env.VITE_ENDPONT;

const useGetById = () => {
  const [data, setData] = useState<any>();
  const fetcher = async (id: number) => {
    const response = await (await fetch(`${endpoint}/task/${id}`)).json();
    console.log(response);
    setData(() => response);
  };

  return { fetcher, data };
};

export default useGetById;
