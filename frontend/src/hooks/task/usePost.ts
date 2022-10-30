const endpoint = import.meta.env.VITE_ENDPONT;

const usePost = () => {
  const request = async () =>
    await fetch(`${endpoint}/task/new`, {
      method: "post",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ title: "from frondend" }),
    });

  return { request };
};

export default usePost;
