use rust_web_ui::*;

pub struct Example35;

pub struct Example35Props {}

impl Default for Example35Props {
	fn default() -> Example35Props {
		Example35Props { }
	}
}

impl Widget<'_> for Example35 {
	type Props = Example35Props;

	fn render(mut ctx: Ctx<'_>, props: Example35Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("section", |mut props| {
			props.styles(&[Style::Noop("relative"), Style::Noop("isolate"), Style::Noop("overflow-hidden"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("-z-10"), Style::Noop("bg-[radial-gradient(45rem_50rem_at_top,theme(colors.indigo.100),white)]"), Style::Noop("opacity-20")]);
			});
			props.child("3", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-1/2"), Style::Noop("-z-10"), Style::MarginRight(Size::Exact(64)), Style::Noop("w-[200%]"), Style::Noop("origin-bottom-left"), Style::Noop("skew-x-[-30deg]"), Style::Noop("bg-white"), Style::Noop("shadow-xl"), Style::Noop("shadow-indigo-600/10"), Style::Noop("ring-1"), Style::Noop("ring-indigo-50"), Style::OnScreen(Screen::Small, &[Style::MarginRight(Size::Exact(112))]), Style::OnScreen(Screen::Large, &[Style::MarginRight(Size::Exact(0))]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::MarginRight(Size::Exact(64))]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("origin-center")])]);
			});
			props.child("5", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-4xl")])]);
				
				props.child("1", Dynamic).run("img", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Width(Size::Exact(48))]);
					props.set_attribute("src", "https://tailwindui.com/img/logos/workcation-logo-indigo-600.svg");
					props.set_attribute("alt", "");
				});
				props.child("3", Dynamic).run("figure", |mut props| {
					props.styles(&[Style::MarginTop(Size::Exact(40))]);
					
					props.child("1", Dynamic).run("blockquote", |mut props| {
						props.styles(&[Style::Noop("text-center"), Style::Noop("text-xl"), Style::FontSemibold, Style::Noop("leading-8"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-2xl")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-9")])]);
						
						props.child("1", Dynamic).run("p", |mut props| {
							props.child("0", Label).run(|props| props.text("“Lorem ipsum dolor sit amet consectetur adipisicing elit. Nemo expedita voluptas culpa sapiente alias molestiae. Numquam corrupti in laborum sed rerum et corporis.”"));
						});
					});
					props.child("3", Dynamic).run("figcaption", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(40))]);
						
						props.child("1", Dynamic).run("img", |mut props| {
							props.styles(&[Style::Noop("mx-auto"), Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::Noop("rounded-full")]);
							props.set_attribute("src", "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
							props.set_attribute("alt", "");
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::SpaceX(Size::Exact(12)), Style::Noop("text-base")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Judith Black"));
							});
							props.child("3", Icon).run(|mut props| {
								props.style(&[Style::Noop("fill-gray-900")]);
							});
							props.child("5", Dynamic).run("div", |mut props| {
								props.styles(&[Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("CEO of Workcation"));
							});
						});
					});
				});
			});
		});
	}
}
