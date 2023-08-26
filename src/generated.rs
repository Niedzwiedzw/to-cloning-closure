
pub trait ToClonedClosure0Args: Sized {
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure0Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move || {
            let tuple = (A0.clone(),);
            builder(tuple)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure0Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move || {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure0Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move || {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure0Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move || {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure0Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move || {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure0Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move || {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure0Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move || {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure0Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move || {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure0Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_0<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move || {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple)
        })
    }
}

pub trait ToClonedClosure1Args: Sized {
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure1Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0| {
            let tuple = (A0.clone(),);
            builder(tuple, C0)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure1Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure1Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure1Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure1Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure1Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure1Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure1Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure1Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_1<R, B, C0>(&self, builder: B) -> Box<dyn (Fn(C0) -> R)>
    where
        B: (Fn(Self, C0) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0)
        })
    }
}

pub trait ToClonedClosure2Args: Sized {
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure2Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0, C1| {
            let tuple = (A0.clone(),);
            builder(tuple, C0, C1)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure2Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0, C1| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0, C1)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure2Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0, C1| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0, C1)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure2Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0, C1| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0, C1)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure2Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0, C1| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0, C1)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure2Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0, C1| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0, C1)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure2Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0, C1| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0, C1)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure2Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0, C1| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0, C1)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure2Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_2<R, B, C0, C1>(&self, builder: B) -> Box<dyn (Fn(C0, C1) -> R)>
    where
        B: (Fn(Self, C0, C1) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0, C1| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0, C1)
        })
    }
}

pub trait ToClonedClosure3Args: Sized {
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure3Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0, C1, C2| {
            let tuple = (A0.clone(),);
            builder(tuple, C0, C1, C2)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure3Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0, C1, C2| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0, C1, C2)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure3Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0, C1, C2| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0, C1, C2)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure3Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0, C1, C2| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0, C1, C2)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure3Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0, C1, C2| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0, C1, C2)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure3Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0, C1, C2| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0, C1, C2)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure3Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0, C1, C2| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0, C1, C2)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure3Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0, C1, C2| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0, C1, C2)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure3Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_3<R, B, C0, C1, C2>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2) -> R)>
    where
        B: (Fn(Self, C0, C1, C2) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0, C1, C2| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0, C1, C2)
        })
    }
}

pub trait ToClonedClosure4Args: Sized {
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure4Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (A0.clone(),);
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure4Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure4Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure4Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure4Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure4Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure4Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure4Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure4Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_4<R, B, C0, C1, C2, C3>(&self, builder: B) -> Box<dyn (Fn(C0, C1, C2, C3) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0, C1, C2, C3| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0, C1, C2, C3)
        })
    }
}

pub trait ToClonedClosure5Args: Sized {
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure5Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (A0.clone(),);
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure5Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure5Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure5Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure5Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure5Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure5Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure5Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure5Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_5<R, B, C0, C1, C2, C3, C4>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4)
        })
    }
}

pub trait ToClonedClosure6Args: Sized {
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure6Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (A0.clone(),);
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure6Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure6Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure6Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure6Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure6Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure6Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure6Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure6Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_6<R, B, C0, C1, C2, C3, C4, C5>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5)
        })
    }
}

pub trait ToClonedClosure7Args: Sized {
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure7Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (A0.clone(),);
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure7Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure7Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure7Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure7Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure7Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure7Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure7Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure7Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_7<R, B, C0, C1, C2, C3, C4, C5, C6>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6)
        })
    }
}

pub trait ToClonedClosure8Args: Sized {
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure8Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (A0.clone(),);
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure8Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure8Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure8Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure8Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure8Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure8Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure8Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure8Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_8<R, B, C0, C1, C2, C3, C4, C5, C6, C7>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7)
        })
    }
}

pub trait ToClonedClosure9Args: Sized {
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static;
}

#[allow(non_snake_case, unused_parens)]
impl<A0> ToClonedClosure9Args for (A0,)
where
    A0: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0,) = self;
        let (A0,) = (A0.clone(),);
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (A0.clone(),);
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1> ToClonedClosure9Args for (A0, A1)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0, A1) = self;
        let (A0, A1) = (A0.clone(), A1.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (A0.clone(), A1.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2> ToClonedClosure9Args for (A0, A1, A2)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0, A1, A2) = self;
        let (A0, A1, A2) = (A0.clone(), A1.clone(), A2.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (A0.clone(), A1.clone(), A2.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3> ToClonedClosure9Args for (A0, A1, A2, A3)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3) = self;
        let (A0, A1, A2, A3) = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4> ToClonedClosure9Args for (A0, A1, A2, A3, A4)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4) = self;
        let (A0, A1, A2, A3, A4) = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (A0.clone(), A1.clone(), A2.clone(), A3.clone(), A4.clone());
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5> ToClonedClosure9Args for (A0, A1, A2, A3, A4, A5)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5) = self;
        let (A0, A1, A2, A3, A4, A5) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6> ToClonedClosure9Args for (A0, A1, A2, A3, A4, A5, A6)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6) = self;
        let (A0, A1, A2, A3, A4, A5, A6) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7> ToClonedClosure9Args for (A0, A1, A2, A3, A4, A5, A6, A7)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}

#[allow(non_snake_case, unused_parens)]
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ToClonedClosure9Args
    for (A0, A1, A2, A3, A4, A5, A6, A7, A8)
where
    A0: Clone + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: Clone + 'static,
    A7: Clone + 'static,
    A8: Clone + 'static,
{
    fn with_cloned_9<R, B, C0, C1, C2, C3, C4, C5, C6, C7, C8>(
        &self,
        builder: B,
    ) -> Box<dyn (Fn(C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R)>
    where
        B: (Fn(Self, C0, C1, C2, C3, C4, C5, C6, C7, C8) -> R) + Copy + 'static,
    {
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = self;
        let (A0, A1, A2, A3, A4, A5, A6, A7, A8) = (
            A0.clone(),
            A1.clone(),
            A2.clone(),
            A3.clone(),
            A4.clone(),
            A5.clone(),
            A6.clone(),
            A7.clone(),
            A8.clone(),
        );
        Box::new(move |C0, C1, C2, C3, C4, C5, C6, C7, C8| {
            let tuple = (
                A0.clone(),
                A1.clone(),
                A2.clone(),
                A3.clone(),
                A4.clone(),
                A5.clone(),
                A6.clone(),
                A7.clone(),
                A8.clone(),
            );
            builder(tuple, C0, C1, C2, C3, C4, C5, C6, C7, C8)
        })
    }
}
