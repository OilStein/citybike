"use client"
import { useState } from "react";
import useSWR from "swr";
import { DataTable } from "@/components/ui/data-table";
import { Station, columns } from "./colunms";
import { Button } from "@/components/ui/button";
import NavBar from "@/components/ui/navbar";

const fetcher = (url: string) => fetch(url).then((res) => res.json());

const StationPage = () => {
  const [page, setPage] = useState(0);
  const { data, error } = useSWR(
  `http://localhost:8080/stations?page=${page}`,
    fetcher
  );
  
  if (error) return <div> Failed to load stations </div>;
  if (!data) return <div> Loading... </div>;
  return (
    <div className="container mx-auto py-5 max-w-[600px]">
      <NavBar></NavBar>
      <DataTable columns={columns} data={data}/>  
      <div className="flex items-center justify-end space-x-2 py-4">
            <Button variant="outline" size="sm" onClick={() => setPage(page - 1)}>
                Previous
            </Button>
            <Button variant="outline" size="sm" onClick={() => setPage(page + 1)}>
                Next
            </Button>
        </div>
    </div>
  );
};

export default StationPage;