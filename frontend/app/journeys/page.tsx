"use client"
import useSWR from "swr";
import { useState } from "react";
import { DataTable } from "@/components/ui/data-table";
import { Journey,columns } from "./columns";
import { Button } from "@/components/ui/button";
import NavBar from "@/components/ui/navbar";


const fetcher = (url: string) => fetch(url).then((res) => res.json());


const previousValidPage = (index: number) => {
    if (index <= 0) {
        return 0
    }
    return index - 1
}

const Table = ({page}: {page: number}) => {
    const { data, error } = useSWR(
        `http://localhost:8080/journeys?page=${page}`,
        fetcher
      );
      if (error) return <div> Failed to load Journeys </div>;
      if (!data) return <div> Loading... </div>;
    
      // console.log(data)
        
    return <DataTable columns={columns} data={data}/> 
}


const JourneyPage = () => {
  const [page, setPage] = useState(0);

  return (
    <div className="container mx-auto py-5">
      <NavBar></NavBar>
        <Table page={page}/>
        <div className="hidden"><Table page={page+1}/></div>
        <div className="flex items-center justify-end space-x-2 py-4">
            <Button variant="outline" size="sm" onClick={() => setPage(previousValidPage(page))}>
                Previous
            </Button>
            <Button variant="outline" size="sm" onClick={() => setPage(page + 1)}>
                Next
            </Button>
        </div>
    </div>
  );
};

export default JourneyPage;