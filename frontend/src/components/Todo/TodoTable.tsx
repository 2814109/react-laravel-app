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
  Spacer,
  Circle,
} from "@chakra-ui/react";
import useGet from "../../hooks/task/useGet";
import useLogicalDelete from "../../hooks/task/useLogicalDelete";
import useUpdate from "../../hooks/task/useUpdate";
import usePhysicalDlete from "../../hooks/task/usePhysicalDelete";
import useGetById from "../../hooks/task/useGetById";
type TaskType = {
  id: number;
  title: string;
  is_closed: boolean;
  created_at: Date;
  updated_at: Date;
};
const TodoList: FC = () => {
  const { data } = useGet();
  const { request: logicalDelete } = useLogicalDelete();
  const { request: update } = useUpdate();
  const { request: physicalDelete } = usePhysicalDlete();
  const { fetcher: getById } = useGetById();
  return (
    <TableContainer>
      <Table variant="simple">
        <TableCaption>Imperial to metric conversion factors</TableCaption>
        <Thead>
          <Tr>
            <Th>id</Th>
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
              <Td>{object.id}</Td>
              <Td>{object.title}</Td>
              <Td>{object.is_closed ? "do it" : "done"}</Td>
              <Td>
                <Center>
                  <Spacer />
                  <Button colorScheme="blue" onClick={() => getById(object.id)}>
                    Show it
                  </Button>
                  <Spacer />
                  <Spacer />
                  <Button colorScheme="green" onClick={() => update()}>
                    Edit
                  </Button>
                  <Spacer />
                  <Button
                    colorScheme="pink"
                    onClick={() => logicalDelete(object.id)}
                  >
                    Did it
                  </Button>
                  <Spacer />
                  <Spacer>
                    <Circle
                      size="40px"
                      bg="red"
                      color="white"
                      onClick={() => physicalDelete(object.id)}
                    >
                      ×
                    </Circle>
                  </Spacer>
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
