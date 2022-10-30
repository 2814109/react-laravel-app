const endpoint = import.meta.env.VITE_ENDPONT;

const useUpdate = () => {
  const request = async () =>
    await fetch(`${endpoint}/task/update`, {
      method: "post",
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify({ id: 5, title: "test update" }),
    });

  return { request };
};

export default useUpdate;
