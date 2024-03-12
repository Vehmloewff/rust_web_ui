use rust_web_ui::*;

pub struct Example42;

pub struct Example42Props {}

impl Default for Example42Props {
	fn default() -> Example42Props {
		Example42Props { }
	}
}

impl Widget<'_> for Example42 {
	type Props = Example42Props;

	fn render(mut ctx: Ctx<'_>, props: Example42Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white"), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::OnScreen(Screen::Large, &[Style::Noop("text-center")])]);
					
					props.child("1", Dynamic).run("h2", |mut props| {
						props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("text-indigo-600")]);
						
						props.child("0", Label).run(|props| props.text("Deploy faster"));
					});
					props.child("3", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.text("Everything you need to deploy your app"));
					});
					props.child("5", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.text("Quis tellus eget adipiscing convallis sit sit eget aliquet quis. Suspendisse eget egestas a elementum pulvinar et feugiat blandit at. In mi viverra elit nunc."));
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::MarginTop(Size::Exact(64)), Style::Noop("max-w-2xl"), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(80))]), Style::OnScreen(Screen::Large, &[Style::MarginTop(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-4xl")])]);
					
					props.child("1", Dynamic).run("dl", |mut props| {
						props.styles(&[Style::Noop("grid"), Style::Noop("max-w-xl"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-10"), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-y-16")])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative"), Style::PaddingLeft(Size::Exact(64))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("absolute"), Style::Noop("left-0"), Style::Noop("top-0"), Style::Flex, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Noop("bg-indigo-600")]);
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("text-white")]);
									});
								});
								props.child("2", Label).run(|props| props.text("Push to deploy"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("Morbi viverra dui mi arcu sed. Tellus semper adipiscing suspendisse semper morbi. Odio urna massa nunc massa."));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative"), Style::PaddingLeft(Size::Exact(64))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("absolute"), Style::Noop("left-0"), Style::Noop("top-0"), Style::Flex, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Noop("bg-indigo-600")]);
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("text-white")]);
									});
								});
								props.child("2", Label).run(|props| props.text("SSL certificates"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("Sit quis amet rutrum tellus ullamcorper ultricies libero dolor eget. Sem sodales gravida quam turpis enim lacus amet."));
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative"), Style::PaddingLeft(Size::Exact(64))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("absolute"), Style::Noop("left-0"), Style::Noop("top-0"), Style::Flex, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Noop("bg-indigo-600")]);
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("text-white")]);
									});
								});
								props.child("2", Label).run(|props| props.text("Simple queues"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("Quisque est vel vulputate cursus. Risus proin diam nunc commodo. Lobortis auctor congue commodo diam neque."));
							});
						});
						props.child("7", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative"), Style::PaddingLeft(Size::Exact(64))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("absolute"), Style::Noop("left-0"), Style::Noop("top-0"), Style::Flex, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Noop("bg-indigo-600")]);
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("text-white")]);
									});
								});
								props.child("2", Label).run(|props| props.text("Advanced security"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("Arcu egestas dolor vel iaculis in ipsum mauris. Tincidunt mattis aliquet hac quis. Id hac maecenas ac donec pharetra eget."));
							});
						});
					});
				});
			});
		});
	}
}
