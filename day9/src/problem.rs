struct FileSystem
{
    m_blocks : Vec<i32>,
}

impl FileSystem
{
    fn from(input: &String) -> Self
    {
        let mut curr_id: i32 = 0;
        let mut blocks : Vec<i32> = Vec::new();

        // consome 2 at a time.
        let mut char_it = input.chars();
        while let Some(first) = char_it.next()
        {
            let second = char_it.next().unwrap_or(' ');

            if let Some(file_size) = first.to_digit(10)
            {
                blocks.extend(std::iter::repeat(curr_id).take(file_size as usize));
            }

            if let Some(blank_gap) = second.to_digit(10)
            {
                blocks.extend(std::iter::repeat(-1).take(blank_gap as usize));
            }

            curr_id += 1;
        }

        Self
        {
            m_blocks: blocks
        }
    }

    fn checksum(&self) -> i64
    {
        let mut result: i64 = 0;

        for it in self.m_blocks.iter().enumerate()
        {
            let block_id: i64 = *it.1 as i64;
            let idx: i64 = it.0 as i64;
            if block_id == -1
            {
                continue;
            }

            result += idx * block_id;
        }

        return result;
    }

    fn defrag(&mut self)
    {
        // Find free head.
        let mut free_head = 0;
        for block in self.m_blocks.iter().enumerate()
        {
            if block.1.clone() == -1
            {
                free_head = block.0;
                break;
            }
        }

        while free_head + 1 < self.m_blocks.len()
        {
            let head = self.m_blocks.pop().expect("Vec ran out of elements?");
            
            assert!(self.m_blocks[free_head] == -1, "Free index not free? {} of {}", free_head, self.m_blocks.len());

            // Move file to first free slot.
            self.m_blocks[free_head] = head;

            // Move free index
            for i in free_head..self.m_blocks.len()
            {
                if self.m_blocks[i] == -1 || i + 1 == self.m_blocks.len()
                {
                    free_head = i;
                    break;
                }
            }
        }
    }

    fn defrag_whole_files(&mut self)
    {
        let (file_list, mut free_list) = self.generate_file_free_lists();

        for file_seg in file_list.iter().rev()
        {
            let file_loc = file_seg.0;
            let file_size = file_seg.1;
            let mut fidx: Option<usize> = None; 
            // Find first eligible free slot.
            for free_seg in free_list.iter().enumerate()
            {
                if free_seg.1.1 >= file_size && free_seg.1.0 < file_loc
                {
                    fidx = Some(free_seg.0);
                    break;
                }
            }

            // No free block found.
            if fidx.is_none()
            {
                continue;
            }

            let fidx = fidx.unwrap();

            let free_seg_start = free_list[fidx].0;
            self.move_file(file_loc, file_size, free_seg_start);

            // Update free list at point we just moved into.
            // Move free index start, forward by size.
            free_list[fidx].0 += file_size;
            // Reduce free segment by size
            free_list[fidx].1 -= file_size;

            // NOTE: because we are going backwards we don't need this!!
            // Now find free index before region we just moved out of.
            // let mut fidx: Option<usize> = None; 
            // for free_seg in free_list.iter().enumerate()
            // {
            //     // This is the block before our free index.
            //     if free_seg.1.0 + free_seg.1.1 == file_loc
            //     {
            //         fidx = Some(free_seg.0);
            //         break;
            //     }
            // }

            // if let Some(fidx) = fidx
            // {
                
            //     // Now expand this free segment.
            //     free_list[fidx].1 += file_size;

            //     // See if we can connect it to the next one.
            //     if fidx < free_list.len() - 1 && free_list[fidx].0 + free_list[fidx].1 == free_list[fidx+1].0
            //     {
            //         // Merge:
            //         // Increase free seg size by block.
            //         free_list[fidx].1 += free_list[fidx+1].1;

            //         free_list.remove(fidx+1);
            //     }
            // }
        }
    }

    // memcpy file and zero out where it was.
    fn move_file(&mut self, mut file_idx: usize, file_size: usize, mut dest: usize)
    {
        //dbg!((file_idx, file_size, dest));
        for _ in 0..file_size
        {
            assert!(self.m_blocks[dest] == -1, "Destination is not free!");
            self.m_blocks[dest] = self.m_blocks[file_idx];
            self.m_blocks[file_idx] = -1;

            file_idx += 1;
            dest += 1;
        }
    }

    fn generate_file_free_lists(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>)
    {
        // Find files and find free list. Format is (start, len)
        let mut free_list : Vec<(usize, usize)> = Vec::new();
        let mut file_list : Vec<(usize, usize)> = Vec::new();

        let mut curr_block_info: (usize, usize) = (0, 0);
        let mut prev_block_id = self.m_blocks[0];
        for it in self.m_blocks.iter().enumerate()
        {
            let curr_block_id = *it.1;

            // Prev block has ended
            if curr_block_id != prev_block_id
            {
                // Write out the info
                match prev_block_id
                {
                    ..0 =>
                    {
                        free_list.push(curr_block_info);
                    }
                    _ =>
                    {
                        file_list.push(curr_block_info);
                    }
                }

                // Start next block
                prev_block_id = curr_block_id;
                curr_block_info.0 = it.0; // Start index
                curr_block_info.1 = 0;
            }

            curr_block_info.1 += 1;
        }

        match prev_block_id
        {
            ..0 =>
            {
                free_list.push(curr_block_info);
            }
            _ =>
            {
                file_list.push(curr_block_info);
            }
        }

        return (file_list, free_list);
    }
}



//
pub fn defrag_and_checksum(input: &String) -> i64
{
    let mut file_system = FileSystem::from(&input);

    file_system.defrag();

    //dbg!(file_system.m_blocks);

    return file_system.checksum();
}

pub fn defrag_whole_files_and_checksum(input: &String) -> i64
{
    let mut file_system = FileSystem::from(&input);

    file_system.defrag_whole_files();
    //dbg!(&file_system.m_blocks);

    return file_system.checksum();
}