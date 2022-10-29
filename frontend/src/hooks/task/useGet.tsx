import { useQuery } from "@tanstack/react-query";
const endpoint = import.meta.env.VITE_ENDPONT;

const useGet = () => {
  const { data } = useQuery(["get-task"], async () => {
    // const data = (await fetch(`${endpoint}/tasks`)).json();
    const data = (await fetch(`${endpoint}/test/3`)).json();

    console.log(String(data));
    return data;
  });
  return { data };
};

export default useGet;
