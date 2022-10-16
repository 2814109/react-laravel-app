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
const TodoList: FC = () => {
  //TODO: to inject by fetch data from api server
  const demoData = [...Array(10)].map((_, index) => {
    const countNumber = index + 1;
    return { id: countNumber, title: `title:${countNumber}`, isClosed: false };
  });

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
          {demoData.map((data) => (
            <Tr key={data.id}>
              <Td>{data.title}</Td>
              <Td>{data.isClosed ? "do it" : "done"}</Td>
              <Td>
                <Center>
                  <Button colorScheme="pink">Did it</Button>
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
