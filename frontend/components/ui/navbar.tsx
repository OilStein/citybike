import Link from "next/link"
import { Button } from "./button"


const NavBar = () => {
    return (
        <div className="container mx-auto py-5">
            <div className="flex justify-center gap-5">
                <Link href={"/stations"}><Button variant="secondary">Stations</Button></Link>
                <Link href={"/"}><Button variant="secondary">Home</Button></Link>
                <Link href={"/journeys"}><Button variant="secondary">Journeys</Button></Link>
            </div>
        </div>
    )
}

export default NavBar