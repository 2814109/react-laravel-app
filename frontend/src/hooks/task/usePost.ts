const endpoint = import.meta.env.VITE_ENDPONT;

const usePost = () => {
  const request = async (title: string) =>
    await fetch(`${endpoint}/task/new`, {
      method: "post",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ title }),
    });

  return { request };
};

export default usePost;
