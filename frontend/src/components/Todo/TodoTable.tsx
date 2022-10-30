import { FC } from "react";
import {
  Table,
  Thead,
  Tbody,
  Tfoot,
  Tr,
  Th,
  Td,
  TableCaption,
  TableContainer,
  Button,
  Center,
} from "@chakra-ui/react";
import useGet from "../../hooks/task/useGet";
import useLogicalDelete from "../../hooks/task/useLogicalDelete";

type TaskType = {
  id: number;
  title: string;
  is_closed: boolean;
  created_at: Date;
  updated_at: Date;
};
const TodoList: FC = () => {
  const { data } = useGet();
  const { request } = useLogicalDelete();

  return (
    <TableContainer>
      <Table variant="simple">
        <TableCaption>Imperial to metric conversion factors</TableCaption>
        <Thead>
          <Tr>
            <Th>Title</Th>
            <Th>Status</Th>
            <Th>
              <Center>Action</Center>
            </Th>
          </Tr>
        </Thead>
        <Tbody>
          {data.task_list.map((object: TaskType) => (
            <Tr key={object.id}>
              <Td>{object.title}</Td>
              <Td>{object.is_closed ? "do it" : "done"}</Td>
              <Td>
                <Center>
                  <Button colorScheme="pink" onClick={() => request()}>
                    Did it
                  </Button>
                </Center>
              </Td>
            </Tr>
          ))}
        </Tbody>
        <Tfoot>
          <Tr>
            <Th>Title</Th>
            <Th>Status</Th>
            <Th>
              <Center>Action</Center>
            </Th>
          </Tr>
        </Tfoot>
      </Table>
    </TableContainer>
  );
};

export default TodoList;
