use bevy::prelude::*;

#[derive(Default, Clone)]
struct Node {
    next: Option<Box<Node>>,
    element: usize,
}

#[derive(Default, Clone)]
struct LinkedList {
    head: Node,
}

impl LinkedList {
    pub fn insert(&mut self, element: usize) {
        let mut node = Node::default();
        node.element = element;

        match self.head.next.clone() {
            Some(next) => {
                node.next = Some(next);
                self.head.next = Some(Box::new(node));
            }
            None => self.head.next = Some(Box::new(node)),
        }
    }

    pub fn as_vec(&self) -> Vec<usize> {
        let mut elements: Vec<usize> = vec![];
        let mut next: Option<Box<Node>> = self.head.next.clone();

        loop {
            match next {
                Some(node) => {
                    let element: usize = node.element;
                    elements.push(element);

                    next = node.next;
                }
                None => break,
            }
        }

        elements
    }
}

#[derive(Clone)]
pub struct Grid {
    pub rect: Rect,
    pub splits: usize,

    cells: Vec<LinkedList>,
}

impl Grid {
    pub fn new(rect: Rect, splits: usize) -> Grid {
        let cell_count: usize = (splits + 1) ^ 2;
        let cells: Vec<LinkedList> = vec![LinkedList::default(); cell_count];

        return Grid {
            rect,
            splits,
            cells,
        };
    }

    pub fn insert(&mut self, cell: usize, element: usize) {
        if !(self.cells.len() > cell) {
            return;
        }

        self.cells[cell].insert(element);

        println!("{:?}, {:?}", cell, element);
    }

    pub fn cleanup(&mut self) {
        self.cells.fill(LinkedList::default());
    }

    pub fn get_cell(&self, position: Vec2) -> usize {
        let min: Vec2 = self.rect.min;
        let max: Vec2 = self.rect.max;

        let position: Vec2 = position.clamp(min, max) - min;
        let mut cell: Vec2 = position / self.rect.size();
        cell *= self.splits as f32 + 1.0;

        let x: usize = cell.x as usize;
        let y: usize = cell.y as usize;

        x + (y * self.splits)
    }

    pub fn get_cell_elements(&self, cell: usize) -> Vec<usize> {
        match self.cells.get(cell) {
            Some(list) => list.as_vec(),
            None => vec![],
        }
    }

    pub fn get_cell_count(&self) -> usize {
        return self.cells.len();
    }
}
