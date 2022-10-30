import { useQuery } from "@tanstack/react-query";
const endpoint = import.meta.env.VITE_ENDPONT;

const useLogicalDelete = () => {
  const request = async () =>
    await fetch(`${endpoint}/task/logical_delete`, {
      method: "post",
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify({ id: 5 }),
    });

  return { request };
};

export default useLogicalDelete;
