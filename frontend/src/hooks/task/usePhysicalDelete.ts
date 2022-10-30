const endpoint = import.meta.env.VITE_ENDPONT;

const usePhysicalDelete = () => {
  const request = async (id: number) =>
    await fetch(`${endpoint}/task/physical_delete`, {
      method: "post",
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify({ id }),
    });

  return { request };
};

export default usePhysicalDelete;
