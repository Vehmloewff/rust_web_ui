use rust_web_ui::*;

pub struct Example43;

pub struct Example43Props {}

impl Default for Example43Props {
	fn default() -> Example43Props {
		Example43Props { }
	}
}

impl Widget<'_> for Example43 {
	type Props = Example43Props;

	fn render(mut ctx: Ctx<'_>, props: Example43Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white"), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::OnScreen(Screen::Small, &[Style::Noop("text-center")])]);
					
					props.child("1", Dynamic).run("h2", |mut props| {
						props.styles(&[Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.text("Simple no-tricks pricing"));
					});
					props.child("3", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.text("Distinctio et nulla eum soluta et neque labore quibusdam. Saepe et quasi iusto modi velit ut non voluptas in. Explicabo id ut laborum."));
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::MarginTop(Size::Exact(64)), Style::Noop("max-w-2xl"), Style::Noop("rounded-3xl"), Style::Noop("ring-1"), Style::Noop("ring-gray-200"), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(80))]), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Flex]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Padding(Size::Exact(32)), Style::OnScreen(Screen::Small, &[Style::Padding(Size::Exact(40))]), Style::OnScreen(Screen::Large, &[Style::Noop("flex-auto")])]);
						
						props.child("1", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Lifetime membership"));
						});
						props.child("3", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
							
							props.child("0", Label).run(|props| props.text("Lorem ipsum dolor sit amet consect etur adipisicing elit. Itaque amet indis perferendis blanditiis repellendus etur quidem assumenda."));
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(40)), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("h4", |mut props| {
								props.styles(&[Style::Noop("flex-none"), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-indigo-600")]);
								
								props.child("0", Label).run(|props| props.text("What’s included"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("h-px"), Style::Noop("flex-auto"), Style::Color(Color::Fg(11))]);
							});
						});
						props.child("7", Dynamic).run("ul", |mut props| {
							props.set_attribute("role", "list");
							props.styles(&[Style::MarginTop(Size::Exact(32)), Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-4"), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67)), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-6")])]);
							
							props.child("1", Dynamic).run("li", |mut props| {
								props.styles(&[Style::Flex, Style::Noop("gap-x-3")]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::Noop("text-indigo-600")]);
								});
								props.child("2", Label).run(|props| props.text("Private forum access"));
							});
							props.child("3", Dynamic).run("li", |mut props| {
								props.styles(&[Style::Flex, Style::Noop("gap-x-3")]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::Noop("text-indigo-600")]);
								});
								props.child("2", Label).run(|props| props.text("Member resources"));
							});
							props.child("5", Dynamic).run("li", |mut props| {
								props.styles(&[Style::Flex, Style::Noop("gap-x-3")]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::Noop("text-indigo-600")]);
								});
								props.child("2", Label).run(|props| props.text("Entry to annual conference"));
							});
							props.child("7", Dynamic).run("li", |mut props| {
								props.styles(&[Style::Flex, Style::Noop("gap-x-3")]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::Noop("text-indigo-600")]);
								});
								props.child("2", Label).run(|props| props.text("Official member t-shirt"));
							});
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("-mt-2"), Style::Padding(Size::Exact(8)), Style::OnScreen(Screen::Large, &[Style::MarginTop(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-md")]), Style::OnScreen(Screen::Large, &[Style::Noop("flex-shrink-0")])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("rounded-2xl"), Style::Color(Color::Fg(6)), Style::PaddingY(Size::Exact(40)), Style::Noop("text-center"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-900/5"), Style::OnScreen(Screen::Large, &[Style::Flex]), Style::OnScreen(Screen::Large, &[Style::Noop("flex-col")]), Style::OnScreen(Screen::Large, &[Style::JustifyCenter]), Style::OnScreen(Screen::Large, &[Style::PaddingY(Size::Exact(64))])]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-xs"), Style::PaddingX(Size::Exact(32))]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.text("Pay once, own it forever"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Noop("items-baseline"), Style::JustifyCenter, Style::Noop("gap-x-2")]);
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("text-5xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("$349"));
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("tracking-wide"), Style::TextColor(Color::Fg(67))]);
										
										props.child("0", Label).run(|props| props.text("USD"));
									});
								});
								props.child("5", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::MarginTop(Size::Exact(40)), Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-center"), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Noop("bg-indigo-500")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-600")])]);
									
									props.child("0", Label).run(|props| props.text("Get access"));
								});
								props.child("7", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.text("Invoices and receipts available for easy company reimbursement"));
								});
							});
						});
					});
				});
			});
		});
	}
}
