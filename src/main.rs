// func gradientDescent(objective func(float32) float32, arg, lr, eps, tol float32, maxIter int) float32 {
// 	x := arg
// 	for i := 0; i < maxIter; i++ {
// 		grad := (objective(x+eps) - objective(x)) / eps
// 		x = x - lr*grad
// 	}
// 	return x
// }

// http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.57.5612&rep=rep1&type=pdf
// def momentum(func, lr, arg, momentum=0.1, eps=0.001, tol=10e-8, max_iter=100):
// func momentum {}

// http://www.cis.pku.edu.cn/faculty/vision/zlin/1983-A%20Method%20of%20Solving%20a%20Convex%20Programming%20Problem%20with%20Convergence%20Rate%20O(k%5E(-2))_Nesterov.pdf
// def nesterov(func, lr, arg, momentum=0.1, eps=0.001, tol=10e-8, max_iter=100):
// func nesterov {}

// http://www.jmlr.org/papers/volume12/duchi11a/duchi11a.pdf
// def adagrad(func, lr, arg, smoothing=1e-8, eps=0.001, tol=10e-8, max_iter=100):
// func adagrad {}

// https://arxiv.org/pdf/1212.5701.pdf
// def adadelta(func, arg, decay=0.9, smoothing=1e-8, eps=0.001, tol=10e-8, max_iter=100):
// func adadelta {}

// https://arxiv.org/pdf/1412.6980.pdf
// adam

// https://arxiv.org/pdf/1412.6980.pdf (yes, same paper)
// adamax

// http://cs229.stanford.edu/proj2015/054_report.pdf
// nadam

// func main() {
// 	f := func(x float32) float32 {
// 		// TODO AS: Seems a bit idiotic, isn't it?
// 		return float32(math.Pow(float64(x), 2) + 5)
// 	}

// 	fmt.Println(gradientDescent(f, 10, 0.2, 0.001, 1e-8, 100))
// 	// print(momentum(f, 0.2, 10))
// 	// print(nesterov(f, 0.2, 10))
// 	// print(adagrad(f, 0.2, 10))
// 	// print(adadelta(f, 10)
// }

// fn gradient_descent(objective: Fn(f32) -> f32) {
//   println!("hola");
// }

fn gradient_descent<F>(objective: F, arg: f32, lr: f32, eta: f32, n_iter: i32) -> f32
where
    F: Fn(f32) -> f32,
{
    let mut x = arg;
    for _ in 0..n_iter {
        x = x - lr * (objective(x + eta) - objective(x - eta)) / (2f32 * eta);
        println!("{:?}", x);
    }
    return x;
}

fn objective(x: f32) -> f32 {
    return x.powi(5) + 5f32;
}

fn main() {
    println!("hola");
    println!("{:?}", objective(2f32));
    println!("{:?}", gradient_descent(objective, 0.1, 0.1, 0.1, 100));
}
