import { useQuery } from "@tanstack/react-query";
const endpoint = import.meta.env.VITE_ENDPONT;

const useGetTasks = () => {
  const { data } = useQuery(["get-task"], async () => {
    const data = (await fetch(`${endpoint}/tasks`)).json();
    console.log(String(data));
    return data;
  });
  return { data };
};

export default useGetTasks;
