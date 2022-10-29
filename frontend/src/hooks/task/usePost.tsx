import { useQuery } from "@tanstack/react-query";
const endpoint = import.meta.env.VITE_ENDPONT;

const usePost = () => {
  const request = async () =>
    await fetch(`${endpoint}/task/new`, {
      method: "post",
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify({ title: "from frondend" }),
    });
  //   const { data } = useQuery(["get-task"], async () => {
  //     const data = (await fetch(`${endpoint}/task`, { method: "post" })).json();
  //     console.log(String(data));
  //     return data;
  //   });
  //   return { data };
  return { request };
};

export default usePost;
